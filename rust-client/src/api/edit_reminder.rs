//! Edit an existing reminder.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
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
    ctx: &ContextSession,
    service_map: &Value,
    options: EditReminderOptions,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<EditReminderResponse> {
    let utils = FactoryUtils::new(ctx);
    let is_group = matches!(thread_type, ThreadType::Group);
    let path = if is_group {
        "/api/board/topic/updatev2"
    } else {
        "/api/board/oneone/update"
    };
    let base_url = service_url(service_map, "group_board", path)?;
    let url = utils.make_url(&base_url, &[], true)?;

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
            "imei": ctx.imei,
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

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
