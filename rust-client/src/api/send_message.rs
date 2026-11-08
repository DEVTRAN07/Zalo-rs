//! Send message endpoint (text messages).

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

/// Send a text message.
///
/// `thread_type`: 0 = user, 1 = group.
pub async fn send_message(
    ctx: &ContextSession,
    service_map: &Value,
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

    let base_url = service_url(service_map, service, path)?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

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
            "imei": ctx.imei,
        })
    } else {
        serde_json::json!({
            "toId": thread_id,
            "message": message,
            "clientId": client_id,
            "imei": ctx.imei,
            "mentionInfo": "",
        })
    };

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
