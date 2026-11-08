//! Add unread mark to a conversation.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddUnreadMarkResponse {
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub status: i64,
}

/// Add unread mark to a conversation.
pub async fn add_unread_mark(
    client: &ApiClient,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<AddUnreadMarkResponse> {
    let url = client.service_url("conversation", "/api/conv/addUnreadMark")?;
    let url = client.make_url(&url, &[], true)?;

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;
    let ts_str = ts.to_string();
    let is_group = matches!(thread_type, ThreadType::Group);
    let key = if is_group { "convsGroup" } else { "convsUser" };
    let other = if is_group { "convsUser" } else { "convsGroup" };

    let param = serde_json::json!({
        key: [{
            "id": thread_id,
            "cliMsgId": ts_str,
            "fromUid": "0",
            "ts": ts,
        }],
        other: [],
        "imei": client.ctx().imei,
    });

    client.post_encrypted(&url, &param).await
}
