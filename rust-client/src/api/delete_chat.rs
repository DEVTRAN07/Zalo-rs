//! Delete a conversation.

use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteChatResponse {
    #[serde(default)]
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChatLastMessage {
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "cliMsgId")]
    pub cli_msg_id: String,
    #[serde(rename = "globalMsgId")]
    pub global_msg_id: String,
}

/// Delete a conversation.
pub async fn delete_chat(
    client: &ApiClient,
    last_message: DeleteChatLastMessage,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<DeleteChatResponse> {
    let is_group = matches!(thread_type, ThreadType::Group);
    let (service, path) = if is_group {
        ("group", "/api/group/deleteconver")
    } else {
        ("chat", "/api/message/deleteconver")
    };
    let base = client.service_url(service, path)?;
    let url = client.make_url(&base, &[("nretry", "0".to_string())], true)?;

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string();

    let conver = serde_json::json!({
        "ownerId": last_message.owner_id,
        "cliMsgId": last_message.cli_msg_id,
        "globalMsgId": last_message.global_msg_id,
    });

    let mut params = serde_json::json!({
        "cliMsgId": ts,
        "conver": conver,
        "onlyMe": 1,
        "imei": client.ctx().imei,
    });
    if is_group {
        params["grid"] = serde_json::Value::String(thread_id.to_string());
    } else {
        params["toid"] = serde_json::Value::String(thread_id.to_string());
    }

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let res = client.request_post(&url, &[("params", encrypted)]).await?;
    client.resolve(res).await
}
