//! Send message endpoint (text messages).

use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

/// Send a text message.
///
/// `thread_type`: 0 = user, 1 = group.
pub async fn send_message(
    client: &ApiClient,
    message: &str,
    thread_id: &str,
    thread_type: u8,
) -> Result<Value> {
    if message.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing message content")));
    }
    if thread_id.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing thread_id")));
    }

    let is_group = thread_type == 1;
    let (service, path) = if is_group {
        ("group", "/api/group/sendmsg")
    } else {
        ("chat", "/api/message/sms")
    };

    let url = client.service_url(service, path)?;
    let url = client.make_url(&url, &[], true)?;

    // Minimal payload for text message; Zalo's full payload includes client ids,
    // mention info, etc. This covers the core protocol.
    let client_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let params = if is_group {
        serde_json::json!({
            "grid": thread_id,
            "message": message,
            "clientId": client_id,
            "visibility": 0,
            "imei": client.ctx().imei,
        })
    } else {
        serde_json::json!({
            "toId": thread_id,
            "message": message,
            "clientId": client_id,
            "imei": client.ctx().imei,
            "mentionInfo": "",
        })
    };

    client.post_encrypted(&url, &params).await
}
