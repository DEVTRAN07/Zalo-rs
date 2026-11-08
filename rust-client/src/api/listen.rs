use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::ws::Listener;

/// Options for starting the WebSocket listener.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListenOptions {
    /// Whether to retry on close (handled internally by `Listener::start` reconnect loop).
    #[serde(default)]
    pub retry_on_close: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListenResponse(pub Value);

/// Create and start a WebSocket listener for real-time events.
///
/// The TypeScript `listen` API wires up a stateful `Listener` (EventEmitter over WS)
/// with callbacks for messages, reactions, typing, etc. In the Rust client, real-time
/// listening is provided by [`crate::ws::Listener`]:
///
/// ```ignore
/// let mut listener = api.listener();
/// let mut rx = listener.start()?;
/// while let Some(event) = rx.recv().await { /* handle */ }
/// listener.stop().await;
/// ```
///
/// This function exists as a compatibility stub and returns an informative error.
/// Use `ZaloApi::listener()` / `ws::Listener` directly for WebSocket event handling.
pub async fn listen(
    _client: &ApiClient,
    _options: Option<ListenOptions>,
) -> Result<ListenResponse> {
    // The actual listener requires WS URLs from ZaloApi context; expose via ZaloApi::listener().
    // Returning a stub error keeps the module compiling with a clear signature.
    Err(crate::errors::ZaloError::Api(crate::errors::ZaloApiError::new(
        "listen() is a WebSocket streaming API; use ZaloApi::listener() -> ws::Listener instead",
    )))
}

/// Helper: construct a `ws::Listener` from client context + WS URLs.
/// This mirrors the TS `listen` factory's setup step.
pub fn create_listener(client: &ApiClient, ws_urls: Vec<String>) -> Listener {
    Listener::new(client.ctx().clone(), ws_urls)
}
