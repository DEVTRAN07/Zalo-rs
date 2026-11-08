//! Create a poll in a group.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePollOptions {
    pub question: String,
    pub options: Vec<String>,
    #[serde(rename = "expiredTime", default)]
    pub expired_time: i64,
    #[serde(rename = "allowMultiChoices", default)]
    pub allow_multi_choices: bool,
    #[serde(rename = "allowAddNewOption", default)]
    pub allow_add_new_option: bool,
    #[serde(rename = "hideVotePreview", default)]
    pub hide_vote_preview: bool,
    #[serde(rename = "isAnonymous", default)]
    pub is_anonymous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePollResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Create a poll in a group.
pub async fn create_poll(
    ctx: &ContextSession,
    service_map: &Value,
    options: CreatePollOptions,
    group_id: &str,
) -> Result<CreatePollResponse> {
    let base_url = service_url(service_map, "group", "/api/poll/create")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "group_id": group_id,
        "question": options.question,
        "options": options.options,
        "expired_time": options.expired_time,
        "pinAct": false,
        "allow_multi_choices": options.allow_multi_choices,
        "allow_add_new_option": options.allow_add_new_option,
        "is_hide_vote_preview": options.hide_vote_preview,
        "is_anonymous": options.is_anonymous,
        "poll_type": 0,
        "src": 1,
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
