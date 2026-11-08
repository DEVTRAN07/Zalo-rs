use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendVoiceOptions {
    #[serde(rename = "voiceUrl")]
    pub voice_url: String,
    pub ttl: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendVoiceResponse {
    #[serde(rename = "msgId", default)]
    pub msg_id: Value,
}

/// Send a voice message to a user or group.
pub async fn send_voice(
    client: &ApiClient,
    options: SendVoiceOptions,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<SendVoiceResponse> {
    if options.voice_url.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing voiceUrl")));
    }

    let file_size: i64 = match client.request_get(&options.voice_url).await {
        Ok(res) => res
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        Err(_) => 0,
    };

    let client_id = chrono::Utc::now().timestamp_millis().to_string();
    let msg_info = serde_json::json!({
        "voiceUrl": options.voice_url,
        "m4aUrl": options.voice_url,
        "fileSize": file_size,
    });

    let is_group = matches!(thread_type, ThreadType::Group);
    let base_url = client.service_url(
        "file",
        if is_group { "/api/group/forward" } else { "/api/message/forward" },
    )?;
    let url = client.make_url(&base_url, &[], true)?;

    let mut params = serde_json::json!({
        "ttl": options.ttl.unwrap_or(0),
        "zsource": -1,
        "msgType": 3,
        "clientId": client_id,
        "msgInfo": msg_info.to_string(),
        "imei": client.ctx().imei,
    });
    if is_group {
        params["grid"] = Value::String(thread_id.to_string());
        params["visibility"] = Value::Number(0.into());
    } else {
        params["toId"] = Value::String(thread_id.to_string());
    }

    client.post_encrypted(&url, &params).await
}
