//! Get group chat history.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupChatHistoryResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get group chat history.
pub async fn get_group_chat_history(
    client: &ApiClient,
    group_id: &str,
    last_id: Option<i64>,
    count: Option<i32>,
) -> Result<GetGroupChatHistoryResponse> {
    let url = client.service_url("group", "/api/group/getmsgv2")?;
    let params = serde_json::json!({
        "grid": group_id,
        "lastId": last_id.unwrap_or(0),
        "count": count.unwrap_or(50),
    });

    client.post_encrypted(&url, &params).await
}
