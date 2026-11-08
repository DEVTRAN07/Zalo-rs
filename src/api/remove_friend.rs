use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type RemoveFriendResponse = Value;

/// Remove friend.
pub async fn remove_friend(client: &ApiClient, friend_id: &str) -> Result<RemoveFriendResponse> {
    let base_url = client.service_url("friend", "/api/friend/remove")?;
    let url = client.make_url(&base_url, &[], true)?;
    let params = serde_json::json!({ "fid": friend_id, "imei": client.ctx().imei });
    client.post_encrypted(&url, &params).await
}
