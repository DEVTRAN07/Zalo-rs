use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::r#enum::ThreadType;

pub type RemoveReminderResponse = Value;

/// Remove a reminder from a group.
pub async fn remove_reminder(
    client: &ApiClient,
    topic_id: &str,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<RemoveReminderResponse> {
    let base_url = client.service_url("group_board", "/api/board/topic/remove")?;
    let url = client.make_url(&base_url, &[], true)?;

    let ttype = match thread_type {
        ThreadType::User => 0,
        ThreadType::Group => 1,
    };
    let params = serde_json::json!({
        "grid": thread_id,
        "topicId": topic_id,
        "topicType": 0,
        "imei": client.ctx().imei,
        "groupType": ttype,
    });
    client.post_encrypted(&url, &params).await
}
