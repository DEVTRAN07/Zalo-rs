use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UndoPayload {
    #[serde(rename = "msgId")]
    pub msg_id: Value,
    #[serde(rename = "cliMsgId")]
    pub cli_msg_id: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UndoResponse {
    #[serde(default)]
    pub status: i64,
}

/// Undo a message.
pub async fn undo(
    client: &ApiClient,
    payload: UndoPayload,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<UndoResponse> {
    let is_group = matches!(thread_type, ThreadType::Group);
    let (service, path) = if is_group {
        ("group", "/api/group/undomsg")
    } else {
        ("chat", "/api/message/undo")
    };
    let url = client.service_url(service, path)?;

    let client_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let mut params = serde_json::json!({
        "msgId": payload.msg_id,
        "clientId": client_id,
        "cliMsgIdUndo": payload.cli_msg_id,
    });

    if is_group {
        params["grid"] = Value::String(thread_id.to_string());
        params["visibility"] = Value::from(0);
        params["imei"] = Value::String(client.ctx().imei.clone());
    } else {
        params["toid"] = Value::String(thread_id.to_string());
    }

    client.post_encrypted(&url, &params).await
}
