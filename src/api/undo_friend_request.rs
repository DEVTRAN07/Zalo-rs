use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type UndoFriendRequestResponse = Value;

/// Undo send a friend request to a user.
pub async fn undo_friend_request(
    client: &ApiClient,
    friend_id: &str,
) -> Result<UndoFriendRequestResponse> {
    let params = serde_json::json!({
        "fid": friend_id,
    });
    let url = client.service_url("friend", "/api/friend/undo")?;
    client.post_encrypted(&url, &params).await
}
