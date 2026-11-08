//! Find user by phone number.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
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
    client: &ApiClient,
    phone_number: &str,
    avatar_size: Option<AvatarSize>,
) -> Result<FindUserResponse> {
    if phone_number.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing phoneNumber")));
    }
    let mut phone = phone_number.to_string();
    if phone.starts_with('0') && client.ctx().language == "vi" {
        phone = format!("84{}", &phone[1..]);
    }
    let avatar = avatar_size.unwrap_or(AvatarSize::Large) as i64;

    let url = client.service_url("friend", "/api/friend/profile/get")?;

    let params = serde_json::json!({
        "phone": phone,
        "avatar_size": avatar,
        "language": client.ctx().language,
        "imei": client.ctx().imei,
        "reqSrc": 40,
    });

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt message")))?;
    let url = client.make_url(&url, &[("params", encrypted.clone())], true)?;
    let res = client.request_get(&url).await?;

    let raw: Value = client.resolve(res).await?;
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
