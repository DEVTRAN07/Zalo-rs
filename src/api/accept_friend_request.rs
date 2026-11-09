//! Accept a friend request.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AcceptFriendRequestResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Accept a friend request from a user.
pub async fn accept_friend_request(
    client: &ApiClient,
    friend_id: &str,
) -> Result<AcceptFriendRequestResponse> {
    let url = client.service_url("friend", "/api/friend/accept")?;
    let params = serde_json::json!({
        "fid": friend_id,
        "language": client.ctx().language,
    });
    client.post_encrypted(&url, &params).await
}
