use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VotePollResponse {
    #[serde(default)]
    pub options: Vec<Value>,
}

/// Vote on a poll. Pass an empty list to unvote.
pub async fn vote_poll(
    client: &ApiClient,
    poll_id: i64,
    option_ids: Vec<i64>,
) -> Result<VotePollResponse> {
    let params = serde_json::json!({
        "poll_id": poll_id,
        "option_ids": option_ids,
        "imei": client.ctx().imei,
    });

    let base_url = client.service_url("group", "/api/poll/vote")?;
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
