//! Create a poll in a group.

use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::Result;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOption {
    pub content: String,
    pub votes: i64,
    pub voted: bool,
    pub voters: Vec<String>,
    #[serde(rename = "optionId")]
    pub option_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePollResponse {
    #[serde(rename = "pollId")]
    pub poll_id: i64,
    pub question: String,
    pub options: Vec<PollOption>,
    #[serde(rename = "expiredTime")]
    pub expired_time: i64,
    #[serde(rename = "pinAct")]
    pub pin_act: bool,
    #[serde(rename = "allowMultiChoices")]
    pub allow_multi_choices: bool,
    #[serde(rename = "allowAddNewOption")]
    pub allow_add_new_option: bool,
    #[serde(rename = "isHideVotePreview")]
    pub is_hide_vote_preview: bool,
    #[serde(rename = "isAnonymous")]
    pub is_anonymous: bool,
    #[serde(rename = "pollType")]
    pub poll_type: i64,
    pub src: i64,
    pub creator: String,
    pub joined: bool,
    pub closed: bool,
    #[serde(rename = "createdTime")]
    pub created_time: i64,
    #[serde(rename = "updatedTime")]
    pub updated_time: i64,
    #[serde(rename = "numVote")]
    pub num_vote: i64,
}

/// Create a poll in a group.
pub async fn create_poll(
    client: &ApiClient,
    options: CreatePollOptions,
    group_id: &str,
) -> Result<CreatePollResponse> {
    let url = client.service_url("group", "/api/poll/create")?;
    let url = client.make_url(&url, &[], true)?;

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
        "imei": client.ctx().imei,
    });

    client.post_encrypted(&url, &params).await
}
