//! Delete a message.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteMessageResponse {
    #[serde(default)]
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageData {
    #[serde(rename = "cliMsgId")]
    pub cli_msg_id: String,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "uidFrom")]
    pub uid_from: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageDestination {
    pub data: DeleteMessageData,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "type", default)]
    pub thread_type: Option<ThreadType>,
}

/// Delete a message.
pub async fn delete_message(
    client: &ApiClient,
    dest: DeleteMessageDestination,
    only_me: bool,
) -> Result<DeleteMessageResponse> {
    let thread_type = dest.thread_type.unwrap_or(ThreadType::User);
    let is_group = matches!(thread_type, ThreadType::Group);
    let is_self = client.ctx().uid == dest.data.uid_from;

    if is_self && !only_me {
        return Err(ZaloError::Api(ZaloApiError::new(
            "To delete your message for everyone, use undo api instead",
        )));
    }
    if !is_group && !only_me {
        return Err(ZaloError::Api(ZaloApiError::new(
            "Can't delete message for everyone in a private chat",
        )));
    }

    let path = if is_group { "/api/group/deletemsg" } else { "/api/message/delete" };
    let url = client.service_url(if is_group { "group" } else { "chat" }, path)?;
    let url = client.make_url(&url, &[], true)?;

    let client_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let mut params = serde_json::json!({
        "cliMsgId": client_id,
        "msgs": [{
            "cliMsgId": dest.data.cli_msg_id,
            "globalMsgId": dest.data.msg_id,
            "ownerId": dest.data.uid_from,
            "destId": dest.thread_id,
        }],
        "onlyMe": if only_me { 1 } else { 0 },
    });

    if is_group {
        params["grid"] = Value::String(dest.thread_id);
    } else {
        params["toid"] = Value::String(dest.thread_id);
        params["imei"] = Value::String(client.ctx().imei.clone());
    }

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt message")))?;
    let res = client.request_post(&url, &[("params", encrypted)]).await?;
    client.resolve(res).await
}
