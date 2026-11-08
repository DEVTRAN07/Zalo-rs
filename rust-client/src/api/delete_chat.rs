//! Delete a conversation.

use serde::{Deserialize, Serialize};

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
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
    ctx: &ContextSession,
    service_map: &serde_json::Value,
    last_message: DeleteChatLastMessage,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<DeleteChatResponse> {
    let utils = FactoryUtils::new(ctx);
    let is_group = matches!(thread_type, ThreadType::Group);
    let (service, path) = if is_group {
        ("group", "/api/group/deleteconver")
    } else {
        ("chat", "/api/message/deleteconver")
    };
    let base = service_url(service_map, service, path)?;
    let url = utils.make_url(&base, &[("nretry", "0".to_string())], true)?;

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
        "imei": ctx.imei,
    });
    if is_group {
        params["grid"] = serde_json::Value::String(thread_id.to_string());
    } else {
        params["toid"] = serde_json::Value::String(thread_id.to_string());
    }

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
