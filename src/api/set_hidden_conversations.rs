use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

pub type SetHiddenConversationsResponse = Value;

/// Set hidden conversations.
pub async fn set_hidden_conversations(
    client: &ApiClient,
    hidden: bool,
    thread_ids: &[String],
    thread_type: ThreadType,
) -> Result<SetHiddenConversationsResponse> {
    if thread_ids.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("threadId is required")));
    }

    let is_group = if matches!(thread_type, ThreadType::Group) { 1 } else { 0 };
    let threads: Vec<Value> = thread_ids
        .iter()
        .map(|id| serde_json::json!({ "thread_id": id, "is_group": is_group }))
        .collect();
    let threads_str = serde_json::to_string(&threads).unwrap_or_else(|_| "[]".to_string());

    let (add_key, del_key) = if hidden {
        ("add_threads", "del_threads")
    } else {
        ("del_threads", "add_threads")
    };

    let params = serde_json::json!({
        add_key: threads_str,
        del_key: "[]",
        "imei": client.ctx().imei,
    });

    let url = client.service_url("conversation", "/api/hiddenconvers/add-remove")?;
    client.post_encrypted(&url, &params).await
}
