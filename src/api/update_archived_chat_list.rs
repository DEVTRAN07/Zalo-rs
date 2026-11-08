use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateArchivedChatListTarget {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ThreadType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateArchivedChatListResponse {
    #[serde(rename = "needResync", default)]
    pub need_resync: bool,
    #[serde(default)]
    pub version: i64,
}

/// Archive or unarchive conversations.
///
/// `is_archived`: true to archive, false to unarchive.
pub async fn update_archived_chat_list(
    client: &ApiClient,
    is_archived: bool,
    conversations: &[UpdateArchivedChatListTarget],
) -> Result<UpdateArchivedChatListResponse> {
    let ids: Vec<Value> = conversations
        .iter()
        .map(|c| serde_json::json!({ "id": c.id, "type": c.type_ as i32 }))
        .collect();

    let version = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let params = serde_json::json!({
        "actionType": if is_archived { 0 } else { 1 },
        "ids": ids,
        "imei": client.ctx().imei,
        "version": version,
    });

    let url = client.service_url("label", "/api/archivedchat/update")?;
    client.post_encrypted(&url, &params).await
}
