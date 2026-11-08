//! Send a contact card.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_qr;
use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCardOptions {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    pub ttl: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendCardResponse {
    #[serde(rename = "msgId", default)]
    pub msg_id: i64,
}

/// Send a card to a User/Group.
pub async fn send_card(
    ctx: &ContextSession,
    service_map: &Value,
    options: SendCardOptions,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<SendCardResponse> {
    let qr = get_qr::get_qr(ctx, service_map, &[options.user_id.clone()]).await?;
    let qr_code_url = qr.get(&options.user_id).cloned().unwrap_or_default();
    let client_id = chrono::Utc::now().timestamp_millis().to_string();

    let mut msg_info = serde_json::json!({
        "contactUid": options.user_id,
        "qrCodeUrl": qr_code_url,
    });
    if let Some(phone) = &options.phone_number {
        msg_info["phone"] = Value::String(phone.clone());
    }

    let mut params = serde_json::json!({
        "ttl": options.ttl.unwrap_or(0),
        "msgType": 6,
        "clientId": client_id,
        "msgInfo": msg_info.to_string(),
    });

    let path = match thread_type {
        ThreadType::Group => "/api/group/forward",
        ThreadType::User => "/api/message/forward",
    };
    let base_url = service_url(service_map, "file", path)?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    match thread_type {
        ThreadType::Group => {
            params["visibility"] = serde_json::json!(0);
            params["grid"] = Value::String(thread_id.to_string());
        }
        ThreadType::User => {
            params["toId"] = Value::String(thread_id.to_string());
            params["imei"] = Value::String(ctx.imei.clone());
        }
    }

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
