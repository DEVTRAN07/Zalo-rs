//! Get user profile information.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::AvatarSize;

pub use super::url::service_url;

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
///
/// `avatar_size` specifies the avatar size to fetch (default: `AvatarSize::Small` = 120).
pub async fn get_user_info(
    client: &ApiClient,
    user_ids: &[&str],
    avatar_size: AvatarSize,
) -> Result<UserInfoResponse> {
    if user_ids.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing user id")));
    }

    let base_url = client.service_url("profile", "/api/social/friend/getprofiles/v2")?;
    let url = client.make_url(&base_url, &[], true)?;

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

    let phonebook_ver = client
        .ctx()
        .extra_ver
        .as_ref()
        .and_then(|v| v.get("phonebook"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    let params = serde_json::json!({
        "phonebook_version": phonebook_ver,
        "friend_pversion_map": friend_pversion_map,
        "avatar_size": avatar_size as i32,
        "language": client.ctx().language,
        "show_online_status": 1,
        "imei": client.ctx().imei,
    });

    client.post_encrypted(&url, &params).await
}
