//! Get avatar url profile.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::AvatarSize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarUrlInfo {
    pub avatar: String,
}

pub type GetAvatarUrlProfileResponse = HashMap<String, AvatarUrlInfo>;

/// Get avatar url profile.
pub async fn get_avatar_url_profile(
    ctx: &ContextSession,
    service_map: &Value,
    friend_ids: Vec<String>,
    avatar_size: Option<AvatarSize>,
) -> Result<GetAvatarUrlProfileResponse> {
    let base_url = service_url(service_map, "profile", "/api/social/profile/avatar-url")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "friend_ids": friend_ids,
        "avatar_size": avatar_size.unwrap_or(AvatarSize::Large) as i32,
        "srcReq": -1,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_get(&url_with_params(&url, &encrypted)).await?;
    utils.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
