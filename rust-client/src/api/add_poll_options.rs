//! Add options to a poll.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::board::PollOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPollOptionsOption {
    pub voted: bool,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPollOptionsPayload {
    #[serde(rename = "pollId")]
    pub poll_id: i64,
    pub options: Vec<AddPollOptionsOption>,
    #[serde(rename = "votedOptionIds", default)]
    pub voted_option_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddPollOptionsResponse {
    #[serde(default)]
    pub options: Vec<PollOption>,
}

/// Add new options to a poll.
pub async fn add_poll_options(
    ctx: &ContextSession,
    service_map: &Value,
    payload: AddPollOptionsPayload,
) -> Result<AddPollOptionsResponse> {
    let base_url = service_url(service_map, "group", "/api/poll/option/add")?;
    let utils = FactoryUtils::new(ctx);

    let params = serde_json::json!({
        "poll_id": payload.poll_id,
        "new_options": serde_json::to_string(&payload.options).unwrap_or_else(|_| "[]".to_string()),
        "voted_option_ids": payload.voted_option_ids,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = utils.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
