//! Edit an existing reminder.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::r#enum::ThreadType;
use crate::models::reminder::ReminderRepeatMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditReminderOptions {
    pub title: String,
    #[serde(rename = "topicId")]
    pub topic_id: String,
    #[serde(default)]
    pub emoji: Option<String>,
    #[serde(rename = "startTime", default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub repeat: Option<ReminderRepeatMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditReminderResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Edit an existing reminder.
pub async fn edit_reminder(
    client: &ApiClient,
    options: EditReminderOptions,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<EditReminderResponse> {
    let is_group = matches!(thread_type, ThreadType::Group);
    let path = if is_group {
        "/api/board/topic/updatev2"
    } else {
        "/api/board/oneone/update"
    };
    let url = client.service_url("group_board", path)?;
    let url = client.make_url(&url, &[], true)?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;
    let start_time = options.start_time.unwrap_or(now);
    let emoji = options.emoji.unwrap_or_default();
    let repeat_val = options.repeat.map(|r| r as i64).unwrap_or(0);

    let params = if is_group {
        serde_json::json!({
            "grid": thread_id,
            "type": 0,
            "color": -16777216_i64,
            "emoji": emoji,
            "startTime": start_time,
            "duration": -1,
            "params": serde_json::json!({ "title": options.title }).to_string(),
            "topicId": options.topic_id,
            "repeat": repeat_val,
            "imei": client.ctx().imei,
            "pinAct": 2,
        })
    } else {
        serde_json::json!({
            "objectData": serde_json::json!({
                "toUid": thread_id,
                "type": 0,
                "color": -16777216_i64,
                "emoji": emoji,
                "startTime": start_time,
                "duration": -1,
                "params": { "title": options.title },
                "needPin": false,
                "reminderId": options.topic_id,
                "repeat": repeat_val,
            }).to_string(),
        })
    };

    client.post_encrypted(&url, &params).await
}
