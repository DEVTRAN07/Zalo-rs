use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloError};
use crate::models::r#enum::ThreadType;

const MAX_MESSAGES_PER_SEND: usize = 50;

pub type SendDeliveredEventResponse = Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveredMessage {
    #[serde(rename = "msgId", default)]
    pub msg_id: String,
    #[serde(rename = "cliMsgId", default)]
    pub cli_msg_id: String,
    #[serde(rename = "uidFrom", default)]
    pub uid_from: String,
    #[serde(rename = "idTo", default)]
    pub id_to: String,
    #[serde(rename = "msgType", default)]
    pub msg_type: String,
    #[serde(default)]
    pub st: i64,
    #[serde(default)]
    pub at: i64,
    #[serde(default)]
    pub cmd: i64,
    #[serde(default)]
    pub ts: Value,
}

fn parse_ts(ts: &Value) -> i64 {
    if let Some(n) = ts.as_i64() {
        return n;
    }
    if let Some(n) = ts.as_u64() {
        return n as i64;
    }
    if let Some(s) = ts.as_str() {
        return s.parse::<i64>().unwrap_or(0);
    }
    0
}

/// Send message delivered/seen event.
pub async fn send_delivered_event(
    client: &ApiClient,
    is_seen: bool,
    messages: &[DeliveredMessage],
    thread_type: ThreadType,
) -> Result<SendDeliveredEventResponse> {
    if messages.is_empty() || messages.len() > MAX_MESSAGES_PER_SEND {
        return Err(ZaloError::Api(crate::errors::ZaloApiError::new(
            "messages must contain between 1 and 50 messages.",
        )));
    }

    let is_group = matches!(thread_type, ThreadType::Group);

    if is_group {
        let id_to = &messages[0].id_to;
        if !messages.iter().all(|msg| &msg.id_to == id_to) {
            return Err(ZaloError::Api(crate::errors::ZaloApiError::new(
                "All messages must have the same idTo for Group thread",
            )));
        }
    }

    let data: Vec<Value> = messages
        .iter()
        .map(|msg| {
            serde_json::json!({
                "cmi": msg.cli_msg_id,
                "gmi": msg.msg_id,
                "si": msg.uid_from,
                "di": if msg.id_to == client.ctx().uid { "0" } else { &msg.id_to },
                "mt": msg.msg_type,
                "st": if msg.st == 0 { 0 } else { -1 },
                "at": if msg.at == 0 { 0 } else { -1 },
                "cmd": if msg.cmd == 0 { 0 } else { -1 },
                "ts": parse_ts(&msg.ts),
            })
        })
        .collect();

    let mut msg_infos = serde_json::json!({
        "seen": if is_seen { 1 } else { 0 },
        "data": data,
    });

    if is_group {
        msg_infos["grid"] = Value::String(messages[0].id_to.clone());
    }

    let (service, path) = if is_group {
        ("group", "/api/group/deliveredv2")
    } else {
        ("chat", "/api/message/deliveredv2")
    };

    let base_url = client.service_url(service, path)?;
    let url = client.make_url(&base_url, &[], true)?;

    let mut params = serde_json::json!({
        "msgInfos": serde_json::to_string(&msg_infos).unwrap_or_else(|_| "{}".to_string()),
    });

    if is_group {
        params["imei"] = Value::String(client.ctx().imei.clone());
    }

    client.post_encrypted(&url, &params).await
}
