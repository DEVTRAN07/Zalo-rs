use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

pub type RemoveFriendAliasResponse = Value;

/// Remove friend's alias.
pub async fn remove_friend_alias(
    client: &ApiClient,
    friend_id: &str,
) -> Result<RemoveFriendAliasResponse> {
    let base_url = client.service_url("friend", "/api/friend/alias/remove")?;
    let params = serde_json::json!({ "friendId": friend_id });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = url_with_params(&base_url, &encrypted);
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
