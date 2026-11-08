//! Remove a reminder.

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::ThreadType;

pub type RemoveReminderResponse = Value;

/// Remove a reminder in a user/group thread.
pub async fn remove_reminder(
    ctx: &ContextSession,
    service_map: &Value,
    reminder_id: &str,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<RemoveReminderResponse> {
    let path = match thread_type {
        ThreadType::User => "/api/board/oneone/remove",
        ThreadType::Group => "/api/board/topic/remove",
    };
    let base_url = service_url(service_map, "group_board", path)?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = match thread_type {
        ThreadType::User => serde_json::json!({
            "uid": thread_id,
            "reminderId": reminder_id,
        }),
        ThreadType::Group => serde_json::json!({
            "grid": thread_id,
            "topicId": reminder_id,
            "imei": ctx.imei,
        }),
    };
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
