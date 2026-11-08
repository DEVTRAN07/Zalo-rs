//! WebSocket listener for real-time Zalo events.
//!
//! Binary protocol: 4-byte header `[version(1), cmd(2 LE), subCmd(1)]` + JSON payload.

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message as WsMessage;

use crate::context::ContextSession;
use crate::core::utils::{decode_event_data, DecodedEventData};
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::{zlog_error, zlog_info, zlog_verbose, zlog_warn};

/// Close reasons matching the TypeScript implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum CloseReason {
    ManualClosure = 1000,
    AbnormalClosure = 1006,
    DuplicateConnection = 3000,
    KickConnection = 3003,
}

/// Decoded WebSocket payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsPayload {
    pub version: u8,
    pub cmd: u16,
    #[serde(rename = "subCmd")]
    pub sub_cmd: u8,
    pub data: Value,
}

/// Events emitted by the listener.
#[derive(Debug, Clone)]
pub enum ListenerEvent {
    Connected,
    Disconnected { code: u16, reason: String },
    CipherKey(String),
    Message(Value),
    GroupMessage(Value),
    Reaction(Value),
    Control(Value),
    OldMessages(Value),
    SeenDelivered(Value),
    Error(String),
    /// Raw/other payload for unhandled cmd/subCmd combinations.
    Raw(WsPayload),
}

/// WebSocket listener for Zalo real-time events.
pub struct Listener {
    ctx: ContextSession,
    urls: Vec<String>,
    stop_tx: Option<mpsc::Sender<()>>,
}

impl Listener {
    pub fn new(ctx: ContextSession, urls: Vec<String>) -> Self {
        Self {
            ctx,
            urls,
            stop_tx: None,
        }
    }

    /// Start the listener and return a receiver for events.
    ///
    /// Spawns a background task that manages the WebSocket connection.
    pub fn start(&mut self) -> Result<mpsc::UnboundedReceiver<ListenerEvent>> {
        if self.stop_tx.is_some() {
            return Err(ZaloError::Api(ZaloApiError::new("Already started")));
        }
        if self.urls.is_empty() {
            return Err(ZaloError::Api(ZaloApiError::new("No WebSocket URLs available")));
        }

        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
        self.stop_tx = Some(stop_tx);

        let ctx = self.ctx.clone();
        let urls = self.urls.clone();

        tokio::spawn(async move {
            let mut url_idx = 0usize;
            loop {
                let base = &urls[url_idx % urls.len()];
                // Match the TypeScript client: append `t` plus the zpw_ver/zpw_type
                // markers the server expects on the WebSocket handshake URL.
                let base_ctx = crate::context::ContextBase {
                    api_type: ctx.api_type,
                    api_version: ctx.api_version,
                    imei: Some(ctx.imei.clone()),
                    user_agent: Some(ctx.user_agent.clone()),
                    language: Some(ctx.language.clone()),
                    secret_key: Some(ctx.secret_key.clone()),
                    uid: Some(ctx.uid.clone()),
                    options: ctx.options.clone(),
                    http: ctx.http.clone(),
                    login_info: Some(ctx.login_info.clone()),
                    settings: ctx.settings.clone(),
                    extra_ver: ctx.extra_ver.clone(),
                };
                let ws_url = crate::http::make_url(
                    &base_ctx,
                    base,
                    &[("t", now_ms().to_string())],
                    true,
                )
                .unwrap_or_else(|_| {
                    if base.contains('?') {
                        format!("{}&t={}", base, now_ms())
                    } else {
                        format!("{}?t={}", base, now_ms())
                    }
                });

                zlog_info!("Connecting to WebSocket: {}", ws_url);
                let cipher_key: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

                let connect_result = connect_and_run(&ctx, &ws_url, event_tx.clone(), &mut stop_rx, cipher_key).await;

                match connect_result {
                    Ok(should_stop) => {
                        if should_stop {
                            let _ = event_tx.send(ListenerEvent::Disconnected {
                                code: CloseReason::ManualClosure as u16,
                                reason: "stopped".into(),
                            });
                            break;
                        }
                        // Connection closed; try next endpoint after a short delay
                        url_idx = (url_idx + 1) % urls.len();
                        zlog_warn!("WebSocket closed, reconnecting...");
                        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    }
                    Err(e) => {
                        let _ = event_tx.send(ListenerEvent::Error(e.to_string()));
                        url_idx = (url_idx + 1) % urls.len();
                        zlog_warn!("WebSocket error: {}, reconnecting...", e);
                        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
                    }
                }
            }
        });

        Ok(event_rx)
    }

    /// Stop the listener.
    pub async fn stop(&mut self) {
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(()).await;
        }
    }
}

async fn connect_and_run(
    ctx: &ContextSession,
    ws_url: &str,
    event_tx: mpsc::UnboundedSender<ListenerEvent>,
    stop_rx: &mut mpsc::Receiver<()>,
    cipher_key: Arc<Mutex<Option<String>>>,
) -> Result<bool> {
    // Build request with headers matching the TypeScript client exactly.
    let mut request = ws_url.into_client_request().map_err(|e| ZaloError::WebSocket(e.to_string()))?;
    let headers = request.headers_mut();
    headers.insert("Origin", HeaderValue::from_static("https://chat.zalo.me"));
    if let Ok(ua) = HeaderValue::from_str(&ctx.user_agent) {
        headers.insert("User-Agent", ua);
    }
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br, zstd"));
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert("Pragma", HeaderValue::from_static("no-cache"));
    headers.insert("Sec-WebSocket-Extensions", HeaderValue::from_static("permessage-deflate; client_max_window_bits"));
    // Cookie header: extract from cookie store
    if let Some(cookie_str) = get_cookie_header(ctx).await {
        if let Ok(v) = HeaderValue::from_str(&cookie_str) {
            headers.insert("Cookie", v);
        }
    }
    // Explicit Host header (some proxies/servers reject requests without it)
    // Note: tokio-tungstenite already sets Host from the URL; re-setting it here
    // can produce a duplicate header on some servers, so we leave it alone.

    let (ws_stream, _) = tokio_tungstenite::connect_async(request)
        .await
        .map_err(|e| ZaloError::WebSocket(e.to_string()))?;

    let _ = event_tx.send(ListenerEvent::Connected);
    zlog_info!("WebSocket connected");

    let (mut write, mut read) = ws_stream.split();
    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    let ping_interval_ms = get_ping_interval_ms(ctx);
    let mut ping_interval = tokio::time::interval(std::time::Duration::from_millis(ping_interval_ms));
    ping_interval.tick().await; // first tick is immediate

    // Writer task
    let writer = tokio::spawn(async move {
        while let Some(data) = out_rx.recv().await {
            if write.send(WsMessage::Binary(data.into())).await.is_err() {
                break;
            }
        }
    });

    loop {
        tokio::select! {
            _ = stop_rx.recv() => {
                let _ = out_tx.send(encode_payload(1, 3, 1, &serde_json::json!({"reason": "manual"})));
                writer.abort();
                return Ok(true);
            }
            _ = ping_interval.tick() => {
                let payload = encode_payload(1, 2, 1, &serde_json::json!({"eventId": now_ms()}));
                if out_tx.send(payload).is_err() {
                    writer.abort();
                    return Ok(false);
                }
            }
            msg = read.next() => {
                let Some(msg) = msg else {
                    writer.abort();
                    return Ok(false);
                };
                match msg {
                    Ok(WsMessage::Binary(data)) => {
                        handle_binary_message(&data, &cipher_key, &event_tx, &out_tx).await;
                    }
                    Ok(WsMessage::Close(frame)) => {
                        zlog_verbose!("WS close frame: {:?}", frame);
                        writer.abort();
                        return Ok(false);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        zlog_error!("WS read error: {}", e);
                        writer.abort();
                        return Err(ZaloError::WebSocket(e.to_string()));
                    }
                }
            }
        }
    }
}

async fn handle_binary_message(
    data: &[u8],
    cipher_key: &Arc<Mutex<Option<String>>>,
    event_tx: &mpsc::UnboundedSender<ListenerEvent>,
    out_tx: &mpsc::UnboundedSender<Vec<u8>>,
) {
    if data.len() < 4 {
        return;
    }
    let version = data[0];
    let cmd = u16::from_le_bytes([data[1], data[2]]);
    let sub_cmd = data[3];
    let payload_bytes = &data[4..];

    let parsed_data: Value = if payload_bytes.is_empty() {
        Value::Null
    } else {
        match serde_json::from_slice(payload_bytes) {
            Ok(v) => v,
            Err(e) => {
                zlog_verbose!("Failed to parse WS payload JSON (cmd={}, sub={}): {}", cmd, sub_cmd, e);
                Value::Null
            }
        }
    };

    // Cipher key handshake
    if version == 1 && cmd == 1 && sub_cmd == 1 {
        if let Some(key) = parsed_data.get("key").and_then(|v| v.as_str()) {
            let mut guard = cipher_key.lock().await;
            *guard = Some(key.to_string());
            let _ = event_tx.send(ListenerEvent::CipherKey(key.to_string()));
            // Send initial ping
            let _ = out_tx.send(encode_payload(1, 2, 1, &serde_json::json!({"eventId": now_ms()})));
            return;
        }
    }

    // Duplicate connection
    if version == 1 && cmd == 3000 && sub_cmd == 0 {
        zlog_error!("Another connection is opened (duplicate connection)");
        let _ = event_tx.send(ListenerEvent::Error("Duplicate connection".into()));
        return;
    }

    // Decode encrypted event payloads. Mirrors the TypeScript client, which
    // calls decodeEventData on every event cmd before processing.
    let key_guard = cipher_key.lock().await;
    let event_data = if let Value::Object(map) = &parsed_data {
        let encrypt = map.get("encrypt").and_then(|v| v.as_i64()).unwrap_or(0);
        if encrypt > 0 {
            match decode_event_data(map, key_guard.as_deref()).await {
                Ok(Some(DecodedEventData::Object(m))) => Value::Object(m),
                Ok(Some(DecodedEventData::Value(v))) => v,
                Ok(None) | Err(_) => parsed_data.clone(),
            }
        } else {
            parsed_data.clone()
        }
    } else {
        parsed_data.clone()
    };
    drop(key_guard);

    let payload = WsPayload {
        version,
        cmd,
        sub_cmd,
        data: event_data.clone(),
    };

    match (cmd, sub_cmd) {
        (501, 0) | (551, 0) => {
            let _ = event_tx.send(ListenerEvent::Message(event_data));
        }
        (521, 0) => {
            let _ = event_tx.send(ListenerEvent::GroupMessage(event_data));
        }
        (612, _) | (610, _) | (611, _) => {
            let _ = event_tx.send(ListenerEvent::Reaction(event_data));
        }
        (601, 0) => {
            let _ = event_tx.send(ListenerEvent::Control(event_data));
        }
        (510, 1) | (511, 1) => {
            let _ = event_tx.send(ListenerEvent::OldMessages(event_data));
        }
        (502, 0) | (522, 0) | (602, 0) => {
            let _ = event_tx.send(ListenerEvent::SeenDelivered(event_data));
        }
        _ => {
            let _ = event_tx.send(ListenerEvent::Raw(payload));
        }
    }
}

/// Encode a WS payload into the binary format: [ver, cmd_lo, cmd_hi, subCmd] + JSON bytes.
pub fn encode_payload(version: u8, cmd: u16, sub_cmd: u8, data: &Value) -> Vec<u8> {
    let json = serde_json::to_vec(data).unwrap_or_default();
    let cmd_bytes = cmd.to_le_bytes();
    let mut buf = Vec::with_capacity(4 + json.len());
    buf.push(version);
    buf.push(cmd_bytes[0]);
    buf.push(cmd_bytes[1]);
    buf.push(sub_cmd);
    buf.extend_from_slice(&json);
    buf
}

/// Decode the 4-byte header.
pub fn decode_header(buf: &[u8]) -> Option<(u8, u16, u8)> {
    if buf.len() < 4 {
        return None;
    }
    let version = buf[0];
    let cmd = u16::from_le_bytes([buf[1], buf[2]]);
    let sub_cmd = buf[3];
    Some((version, cmd, sub_cmd))
}

async fn get_cookie_header(ctx: &ContextSession) -> Option<String> {
    let guard = ctx.http.lock().await;
    let http = guard.as_ref()?;
    let store = http.cookie_store.lock().ok()?;
    let url = url::Url::parse("https://chat.zalo.me/").ok()?;
    let cookies: Vec<String> = store
        .get_request_values(&url)
        .map(|(name, value)| format!("{}={}", name, value))
        .collect();
    if cookies.is_empty() {
        None
    } else {
        Some(cookies.join("; "))
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Read the ping interval (ms) from `ctx.settings.features.socket.ping_interval`,
/// falling back to the TypeScript default of 30s when unavailable.
fn get_ping_interval_ms(ctx: &ContextSession) -> u64 {
    let default = 30_000u64;
    let Some(settings) = ctx.settings.as_ref() else {
        return default;
    };
    let interval = settings
        .get("features")
        .and_then(|f| f.get("socket"))
        .and_then(|s| s.get("ping_interval"))
        .and_then(|v| v.as_u64())
        .unwrap_or(default);
    if interval == 0 {
        default
    } else {
        interval
    }
}
