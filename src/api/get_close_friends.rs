//! Get close friends.

use crate::api::client::ApiClient;

use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::user::User;

pub type GetCloseFriendsResponse = Vec<User>;

/// Get close friends.
pub async fn get_close_friends(client: &ApiClient) -> Result<GetCloseFriendsResponse> {
    let url = client.service_url("profile", "/api/social/friend/getclosedfriends")?;
    let params = serde_json::json!({});

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
