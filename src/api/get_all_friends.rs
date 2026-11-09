//! Get all friends.

use crate::api::client::ApiClient;

use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::AvatarSize;
use crate::models::user::User;

pub type GetAllFriendsResponse = Vec<User>;

/// Get all friends.
pub async fn get_all_friends(
    client: &ApiClient,
    count: Option<i32>,
    page: Option<i32>,
    avatar_size: Option<AvatarSize>,
) -> Result<GetAllFriendsResponse> {
    let url = client.service_url("profile", "/api/social/friend/getfriends")?;
    let params = serde_json::json!({
        "incInvalid": 1,
        "page": page.unwrap_or(1),
        "count": count.unwrap_or(20000),
        "avatar_size": avatar_size.unwrap_or(AvatarSize::Small) as i32,
        "actiontime": 0,
        "imei": client.ctx().imei,
    });

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt message")))?;
    let full_url = if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(&encrypted))
    } else {
        format!("{}?params={}", url, urlencoding::encode(&encrypted))
    };
    let res = client.request_get(&full_url).await?;
    client.resolve(res).await
}
