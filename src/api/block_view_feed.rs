//! Block/unblock a user from viewing your feed.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockViewFeedResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Block/unblock friend view feed by ID.
pub async fn block_view_feed(
    client: &ApiClient,
    is_block_feed: bool,
    user_id: &str,
) -> Result<BlockViewFeedResponse> {
    let url = client.service_url("friend", "/api/friend/feed/block")?;
    let params = serde_json::json!({
        "fid": user_id,
        "isBlockFeed": if is_block_feed { 1 } else { 0 },
        "imei": client.ctx().imei,
    });
    client.post_encrypted(&url, &params).await
}
