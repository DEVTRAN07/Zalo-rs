//! Block a user.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockUserResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Block a user.
pub async fn block_user(client: &ApiClient, user_id: &str) -> Result<BlockUserResponse> {
    let url = client.service_url("friend", "/api/friend/block")?;
    let params = serde_json::json!({
        "fid": user_id,
        "imei": client.ctx().imei,
    });
    client.post_encrypted(&url, &params).await
}
