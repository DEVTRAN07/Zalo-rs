//! Get all friends.

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::AvatarSize;
use crate::models::user::User;

pub type GetAllFriendsResponse = Vec<User>;

/// Get all friends.
pub async fn get_all_friends(
    ctx: &ContextSession,
    service_map: &Value,
    count: Option<i32>,
    page: Option<i32>,
    avatar_size: Option<AvatarSize>,
) -> Result<GetAllFriendsResponse> {
    let base_url = service_url(service_map, "profile", "/api/social/friend/getfriends")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "incInvalid": 1,
        "page": page.unwrap_or(1),
        "count": count.unwrap_or(20000),
        "avatar_size": avatar_size.unwrap_or(AvatarSize::Small) as i32,
        "actiontime": 0,
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt message")))?;

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
