use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type UnblockUserResponse = Value;

/// Unblock a user.
pub async fn unblock_user(client: &ApiClient, user_id: &str) -> Result<UnblockUserResponse> {
    let params = serde_json::json!({
        "fid": user_id,
        "imei": client.ctx().imei,
    });
    let url = client.service_url("friend", "/api/friend/unblock")?;
    client.post_encrypted(&url, &params).await
}
