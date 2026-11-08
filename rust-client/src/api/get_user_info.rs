//! Get user profile information.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserInfoResponse {
    #[serde(default)]
    pub unchanged_profiles: Value,
    #[serde(default)]
    pub phonebook_version: i64,
    #[serde(default)]
    pub changed_profiles: Value,
}

/// Get user info using user id(s).
///
/// `user_ids` are Zalo user IDs. The API expects `{id}_0` format internally.
pub async fn get_user_info(
    ctx: &ContextSession,
    service_map: &Value,
    user_ids: &[&str],
) -> Result<UserInfoResponse> {
    if user_ids.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing user id")));
    }

    let base_url = service_url(service_map, "profile", "/api/social/friend/getprofiles/v2")?;
    let utils = FactoryUtils::new(ctx);
    let service_url = utils.make_url(&base_url, &[], true)?;

    let friend_pversion_map: Vec<String> = user_ids
        .iter()
        .map(|id| {
            if id.contains('_') {
                id.to_string()
            } else {
                format!("{}_0", id)
            }
        })
        .collect();

    let phonebook_ver = ctx
        .extra_ver
        .as_ref()
        .and_then(|v| v.get("phonebook"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    let params = serde_json::json!({
        "phonebook_version": phonebook_ver,
        "friend_pversion_map": friend_pversion_map,
        "avatar_size": 120,
        "language": ctx.language,
        "show_online_status": 1,
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&service_url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}

/// Resolve a service URL from the zpw_service_map.
pub(crate) fn service_url(service_map: &Value, service: &str, path: &str) -> Result<String> {
    let base = service_map
        .get(service)
        .and_then(|v| v.as_array())
        .and_then(|a| a.first())
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            ZaloError::Api(ZaloApiError::new(format!(
                "Missing service URL for '{}' in zpw_service_map",
                service
            )))
        })?;
    let base = base.trim_end_matches('/');
    let path = if path.starts_with('/') { path.to_string() } else { format!("/{}", path) };
    Ok(format!("{}{}", base, path))
}
