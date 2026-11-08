use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type SendFriendRequestResponse = Value;

/// Send a friend request to a user.
pub async fn send_friend_request(
    client: &ApiClient,
    msg: &str,
    user_id: &str,
) -> Result<SendFriendRequestResponse> {
    let url = client.service_url("friend", "/api/friend/sendreq")?;
    let params = serde_json::json!({
        "toid": user_id,
        "msg": msg,
        "reqsrc": 30,
        "imei": client.ctx().imei,
        "language": client.ctx().language,
        "srcParams": serde_json::json!({ "uidTo": user_id }).to_string(),
    });
    client.post_encrypted(&url, &params).await
}
