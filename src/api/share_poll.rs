use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type SharePollResponse = Value;

/// Share a poll.
pub async fn share_poll(client: &ApiClient, poll_id: i64) -> Result<SharePollResponse> {
    let params = serde_json::json!({
        "poll_id": poll_id,
        "imei": client.ctx().imei,
    });
    let url = client.service_url("group", "/api/poll/share")?;
    client.post_encrypted(&url, &params).await
}
