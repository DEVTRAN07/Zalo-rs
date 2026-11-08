//! Find user by phone number.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::AvatarSize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FindUserResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Find user by phone number.
///
/// Normalizes leading `0` to `84` for Vietnamese locale (matching TS behavior).
pub async fn find_user(
    ctx: &ContextSession,
    service_map: &Value,
    phone_number: &str,
    avatar_size: Option<AvatarSize>,
) -> Result<FindUserResponse> {
    if phone_number.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing phoneNumber")));
    }
    let mut phone = phone_number.to_string();
    if phone.starts_with('0') && ctx.language == "vi" {
        phone = format!("84{}", &phone[1..]);
    }
    let avatar = avatar_size.unwrap_or(AvatarSize::Large) as i64;

    let base_url = service_url(service_map, "friend", "/api/friend/profile/get")?;
    let utils = FactoryUtils::new(ctx);

    let params = serde_json::json!({
        "phone": phone,
        "avatar_size": avatar,
        "language": ctx.language,
        "imei": ctx.imei,
        "reqSrc": 40,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt message")))?;
    let url = utils.make_url(&base_url, &[("params", encrypted.clone())], true)?;
    let res = utils.request_get(&url).await?;

    let raw: Value = utils.resolve(res).await?;
    // TS treats error code 216 as "not found but not fatal" in some paths;
    // preserve raw data and let caller interpret.
    if let Some(err) = raw.get("error") {
        if let Some(code) = err.get("code").and_then(|c| c.as_i64()) {
            if code != 0 && code != 216 {
                let msg = err.get("message").and_then(|m| m.as_str()).unwrap_or("find_user error");
                return Err(ZaloError::Api(ZaloApiError::with_code(msg, code)));
            }
        }
    }
    Ok(FindUserResponse { data: raw.get("data").cloned().unwrap_or(raw) })
}
