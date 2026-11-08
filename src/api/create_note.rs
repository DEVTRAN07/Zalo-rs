//! Create a note in a group.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNoteOptions {
    pub title: String,
    #[serde(rename = "pinAct", default)]
    pub pin_act: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateNoteResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Create a note in a group.
pub async fn create_note(
    client: &ApiClient,
    options: CreateNoteOptions,
    group_id: &str,
) -> Result<CreateNoteResponse> {
    let url = client.service_url("group_board", "/api/board/topic/createv2")?;
    let url = client.make_url(&url, &[], true)?;

    let params = serde_json::json!({
        "grid": group_id,
        "type": 0,
        "color": -16777216_i64,
        "emoji": "",
        "startTime": -1,
        "duration": -1,
        "params": serde_json::json!({ "title": options.title }).to_string(),
        "repeat": 0,
        "src": 1,
        "imei": client.ctx().imei,
        "pinAct": if options.pin_act { 1 } else { 0 },
    });

    client.post_encrypted(&url, &params).await
}
