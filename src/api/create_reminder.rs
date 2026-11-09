//! Create a reminder.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::r#enum::ThreadType;
use crate::models::reminder::ReminderRepeatMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReminderOptions {
    pub title: String,
    #[serde(default)]
    pub emoji: Option<String>,
    #[serde(rename = "startTime", default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub repeat: Option<ReminderRepeatMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateReminderResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Create a reminder in a user or group conversation.
pub async fn create_reminder(
    client: &ApiClient,
    options: CreateReminderOptions,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<CreateReminderResponse> {
    let is_group = matches!(thread_type, ThreadType::Group);
    let path = if is_group {
        "/api/board/topic/createv2"
    } else {
        "/api/board/oneone/create"
    };
    let url = client.service_url("group_board", path)?;
    let url = client.make_url(&url, &[], true)?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;
    let start_time = options.start_time.unwrap_or(now);
    let emoji = options.emoji.unwrap_or_else(|| "⏰".to_string());
    let repeat_val = options.repeat.map(|r| r as i64).unwrap_or(0);

    let params = if is_group {
        serde_json::json!({
            "grid": thread_id,
            "type": 0,
            "color": -16245706_i64,
            "emoji": emoji,
            "startTime": start_time,
            "duration": -1,
            "params": serde_json::json!({ "title": options.title }).to_string(),
            "repeat": repeat_val,
            "src": 1,
            "imei": client.ctx().imei,
            "pinAct": 0,
        })
    } else {
        serde_json::json!({
            "objectData": serde_json::json!({
                "toUid": thread_id,
                "type": 0,
                "color": -16245706_i64,
                "emoji": emoji,
                "startTime": start_time,
                "duration": -1,
                "params": { "title": options.title },
                "needPin": false,
                "repeat": repeat_val,
                "creatorUid": client.ctx().uid,
                "src": 1,
            }).to_string(),
            "imei": client.ctx().imei,
        })
    };

    client.post_encrypted(&url, &params).await
}
