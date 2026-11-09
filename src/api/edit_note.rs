//! Edit an existing group note.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

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
    client: &ApiClient,
    options: EditNoteOptions,
    group_id: &str,
) -> Result<EditNoteResponse> {
    let url = client.service_url("group_board", "/api/board/topic/updatev2")?;
    let url = client.make_url(&url, &[], true)?;

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
        "imei": client.ctx().imei,
        "pinAct": if options.pin_act { 1 } else { 2 },
    });

    client.post_encrypted(&url, &params).await
}
