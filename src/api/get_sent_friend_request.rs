use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSentFriendRequestResponse {
    #[serde(rename = "recommItems", default)]
    pub recomm_items: Vec<Value>,
    #[serde(rename = "totalReq", default)]
    pub total_req: i64,
}

/// Get my sent friend requests (undone).
pub async fn get_sent_friend_request(
    client: &ApiClient,
) -> Result<GetSentFriendRequestResponse> {
    let base_url = client.service_url("friend", "/api/friend/requested/list")?;
    let url = client.make_url(&base_url, &[("nretry", "0".to_string())], true)?;
    let params = serde_json::json!({ "imei": client.ctx().imei });
    client.post_encrypted(&url, &params).await
}
