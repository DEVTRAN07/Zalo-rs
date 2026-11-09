use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendVideoOptions {
    pub msg: Option<String>,
    #[serde(rename = "videoUrl")]
    pub video_url: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
    pub duration: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub ttl: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendVideoResponse {
    #[serde(rename = "msgId", default)]
    pub msg_id: Value,
}

/// Send a video to a user or group.
pub async fn send_video(
    client: &ApiClient,
    options: SendVideoOptions,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<SendVideoResponse> {
    if options.video_url.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing videoUrl")));
    }
    if options.thumbnail_url.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing thumbnailUrl")));
    }

    // Best-effort content-length via HEAD (may fail; default to 0).
    let file_size: i64 = match client.request_head(&options.video_url).await {
        Ok(res) => res
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        Err(_) => 0,
    };

    let client_id = chrono::Utc::now().timestamp_millis();
    let msg_info = serde_json::json!({
        "videoUrl": options.video_url,
        "thumbUrl": options.thumbnail_url,
        "duration": options.duration.unwrap_or(0),
        "width": options.width.unwrap_or(1280),
        "height": options.height.unwrap_or(720),
        "fileSize": file_size,
        "properties": {
            "color": -1,
            "size": -1,
            "type": 1003,
            "subType": 0,
            "ext": {
                "sSrcType": -1,
                "sSrcStr": "",
                "msg_warning_type": 0,
            }
        },
        "title": options.msg.unwrap_or_default(),
    });

    let is_group = matches!(thread_type, ThreadType::Group);
    let base_url = client.service_url(
        "file",
        if is_group { "/api/group/forward" } else { "/api/message/forward" },
    )?;
    let url = client.make_url(&base_url, &[], true)?;

    let mut params = serde_json::json!({
        "clientId": client_id.to_string(),
        "ttl": options.ttl.unwrap_or(0),
        "zsource": 704,
        "msgType": 5,
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
