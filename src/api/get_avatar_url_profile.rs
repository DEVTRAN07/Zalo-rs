//! Get avatar url profile.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::AvatarSize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarUrlInfo {
    pub avatar: String,
}

pub type GetAvatarUrlProfileResponse = HashMap<String, AvatarUrlInfo>;

/// Get avatar url profile.
pub async fn get_avatar_url_profile(
    client: &ApiClient,
    friend_ids: Vec<String>,
    avatar_size: Option<AvatarSize>,
) -> Result<GetAvatarUrlProfileResponse> {
    let url = client.service_url("profile", "/api/social/profile/avatar-url")?;
    let params = serde_json::json!({
        "friend_ids": friend_ids,
        "avatar_size": avatar_size.unwrap_or(AvatarSize::Large) as i32,
        "srcReq": -1,
    });

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let full_url = if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(&encrypted))
    } else {
        format!("{}?params={}", url, urlencoding::encode(&encrypted))
    };
    let res = client.request_get(&full_url).await?;
    client.resolve(res).await
}
