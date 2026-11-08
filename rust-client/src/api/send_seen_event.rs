use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendSeenEventResponse {
    #[serde(default)]
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendSeenEventMessageParams {
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

fn ts_to_i64(ts: &Value) -> i64 {
    if let Some(n) = ts.as_i64() {
        n
    } else if let Some(s) = ts.as_str() {
        s.parse::<i64>().unwrap_or(0)
    } else if let Some(n) = ts.as_u64() {
        n as i64
    } else {
        0
    }
}

/// Send message seen event.
///
/// `messages` must contain between 1 and 50 messages and all must belong to the same thread.
pub async fn send_seen_event(
    client: &ApiClient,
    messages: &[SendSeenEventMessageParams],
    thread_type: ThreadType,
) -> Result<SendSeenEventResponse> {
    if messages.is_empty() || messages.len() > 50 {
        return Err(ZaloError::Api(ZaloApiError::new(
            "messages must contain between 1 and 50 messages.",
        )));
    }

    let is_group = matches!(thread_type, ThreadType::Group);
    let (service, path) = if is_group {
        ("group", "/api/group/seenv2")
    } else {
        ("chat", "/api/message/seenv2")
    };
    let base_url = client.service_url(service, path)?;
    let url = client.make_url(&base_url, &[("nretry", "0".to_string())], true)?;

    let thread_id = if is_group {
        messages[0].id_to.clone()
    } else {
        messages[0].uid_from.clone()
    };

    let mut data_items = Vec::with_capacity(messages.len());
    for msg in messages {
        let cur_thread_id = if is_group { &msg.id_to } else { &msg.uid_from };
        if *cur_thread_id != thread_id {
            return Err(ZaloError::Api(ZaloApiError::new(
                "All messages must belong to the same thread.",
            )));
        }
        let st = if msg.st == 0 { 0 } else { -1 };
        let at = if msg.at == 0 { 0 } else { -1 };
        let cmd = if msg.cmd == 0 { 0 } else { -1 };
        data_items.push(serde_json::json!({
            "gmi": msg.msg_id,
            "cmi": msg.cli_msg_id,
            "si": msg.uid_from,
            "di": if msg.id_to == client.ctx().uid { "0" } else { &msg.id_to },
            "mt": msg.msg_type,
            "st": st,
            "at": at,
            "cmd": cmd,
            "ts": ts_to_i64(&msg.ts),
        }));
    }

    let mut msg_infos = serde_json::json!({ "data": data_items });
    if is_group {
        msg_infos["grid"] = Value::String(thread_id);
    } else {
        msg_infos["senderId"] = Value::String(thread_id);
    }

    let mut params = serde_json::json!({
        "msgInfos": msg_infos.to_string(),
    });
    if is_group {
        params["imei"] = Value::String(client.ctx().imei.clone());
    }

    client.post_encrypted(&url, &params).await
}
