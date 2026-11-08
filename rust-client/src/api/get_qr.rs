//! Get QR code for users.

use serde_json::Value;
use std::collections::HashMap;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

pub type GetQrResponse = HashMap<String, String>;

/// Get QR code for users.
pub async fn get_qr(
    ctx: &ContextSession,
    service_map: &Value,
    user_id: &[String],
) -> Result<GetQrResponse> {
    if user_id.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing user id")));
    }
    let base_url = service_url(service_map, "friend", "/api/friend/mget-qr")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({ "fids": user_id });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils
        .request_post(&url, &[("params", encrypted)])
        .await?;
    utils.resolve(res).await
}
