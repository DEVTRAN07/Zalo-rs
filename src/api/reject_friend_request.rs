use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type RejectFriendRequestResponse = Value;

/// Reject friend request.
pub async fn reject_friend_request(
    client: &ApiClient,
    friend_id: &str,
) -> Result<RejectFriendRequestResponse> {
    let base_url = client.service_url("friend", "/api/friend/reject")?;
    let url = client.make_url(&base_url, &[], true)?;
    let params = serde_json::json!({ "fid": friend_id });
    client.post_encrypted(&url, &params).await
}
