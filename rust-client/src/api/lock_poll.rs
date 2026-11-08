use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type LockPollResponse = Value;

/// Lock poll.
pub async fn lock_poll(client: &ApiClient, poll_id: i64) -> Result<LockPollResponse> {
    let base_url = client.service_url("group_board", "/api/board/poll/lock")?;
    let url = client.make_url(&base_url, &[], true)?;
    let params = serde_json::json!({ "pollId": poll_id, "imei": client.ctx().imei });
    client.post_encrypted(&url, &params).await
}
