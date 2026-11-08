//! Get poll detail.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetPollDetailResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get poll detail.
pub async fn get_poll_detail(
    client: &ApiClient,
    poll_id: i64,
) -> Result<GetPollDetailResponse> {
    let url = client.service_url("group_poll", "/api/group/poll/detail")?;
    let params = serde_json::json!({
        "poll_id": poll_id,
    });

    client.post_encrypted(&url, &params).await
}
