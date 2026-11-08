use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveUnreadMarkResponse {
    #[serde(default)]
    pub data: Option<Value>,
}

/// Remove unread mark from conversations.
pub async fn remove_unread_mark(
    client: &ApiClient,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<RemoveUnreadMarkResponse> {
    let base_url = client.service_url("conversation", "/api/conv/removeUnreadMark")?;
    let url = client.make_url(&base_url, &[], true)?;

    let param = serde_json::json!({
        if matches!(thread_type, ThreadType::User) { "friendId" } else { "groupId" }: thread_id
    });
    let data_str = serde_json::to_string(&param).unwrap_or_else(|_| "{}".to_string());
    let params = serde_json::json!({
        "param": data_str,
    });
    client.post_encrypted(&url, &params).await
}
