//! Edit an existing group note.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditNoteOptions {
    pub title: String,
    #[serde(rename = "topicId")]
    pub topic_id: String,
    #[serde(rename = "pinAct", default)]
    pub pin_act: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditNoteResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Edit an existing note in a group.
pub async fn edit_note(
    ctx: &ContextSession,
    service_map: &Value,
    options: EditNoteOptions,
    group_id: &str,
) -> Result<EditNoteResponse> {
    let base_url = service_url(service_map, "group_board", "/api/board/topic/updatev2")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "grid": group_id,
        "type": 0,
        "color": -16777216_i64,
        "emoji": "",
        "startTime": -1,
        "duration": -1,
        "params": serde_json::json!({ "title": options.title }).to_string(),
        "topicId": options.topic_id,
        "repeat": 0,
        "imei": ctx.imei,
        "pinAct": if options.pin_act { 1 } else { 2 },
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
