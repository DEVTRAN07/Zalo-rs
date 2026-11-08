use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendStickerPayload {
    pub id: i64,
    #[serde(rename = "cateId")]
    pub cate_id: i64,
    #[serde(rename = "type")]
    pub sticker_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendStickerResponse {
    #[serde(rename = "msgId", default)]
    pub msg_id: Value,
}

/// Send a sticker to a thread.
pub async fn send_sticker(
    client: &ApiClient,
    sticker: SendStickerPayload,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<SendStickerResponse> {
    if thread_id.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing threadId")));
    }
    if sticker.id == 0 {
        return Err(ZaloError::Api(ZaloApiError::new("Missing sticker id")));
    }
    if sticker.sticker_type == 0 {
        return Err(ZaloError::Api(ZaloApiError::new("Missing sticker type")));
    }

    let is_group = matches!(thread_type, ThreadType::Group);
    let (base_svc, path) = if is_group {
        ("group", "/api/group/sticker")
    } else {
        ("chat", "/api/message/sticker")
    };
    let base_url = client.service_url(base_svc, path)?;
    let url = client.make_url(&base_url, &[("nretry", "0".to_string())], true)?;

    let mut params = serde_json::json!({
        "stickerId": sticker.id,
        "cateId": sticker.cate_id,
        "type": sticker.sticker_type,
        "clientId": chrono::Utc::now().timestamp_millis(),
        "imei": client.ctx().imei,
        "zsource": 101,
    });
    if is_group {
        params["grid"] = Value::String(thread_id.to_string());
    } else {
        params["toid"] = Value::String(thread_id.to_string());
    }

    client.post_encrypted(&url, &params).await
}
