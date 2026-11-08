use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::r#enum::ThreadType;

pub type SetPinnedConversationsResponse = Value;

/// Pin or unpin conversations.
pub async fn set_pinned_conversations(
    client: &ApiClient,
    pinned: bool,
    thread_ids: &[String],
    thread_type: ThreadType,
) -> Result<SetPinnedConversationsResponse> {
    let conversations: Vec<String> = thread_ids
        .iter()
        .map(|id| {
            if matches!(thread_type, ThreadType::Group) {
                format!("g{}", id)
            } else {
                format!("u{}", id)
            }
        })
        .collect();

    let params = serde_json::json!({
        "actionType": if pinned { 1 } else { 2 },
        "conversations": conversations,
    });

    let url = client.service_url("conversation", "/api/pinconvers/updatev2")?;
    client.post_encrypted(&url, &params).await
}
