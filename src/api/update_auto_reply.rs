use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::auto_reply::{AutoReplyItem, AutoReplyScope};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAutoReplyPayload {
    pub id: i64,
    pub content: String,
    #[serde(rename = "isEnable")]
    pub is_enable: bool,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    pub scope: AutoReplyScope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateAutoReplyResponse {
    #[serde(default)]
    pub item: Option<AutoReplyItem>,
    #[serde(default)]
    pub version: i64,
}

/// Update auto reply (zBusiness).
pub async fn update_auto_reply(
    client: &ApiClient,
    payload: UpdateAutoReplyPayload,
) -> Result<UpdateAutoReplyResponse> {
    let result_uids: Vec<String> = match payload.scope {
        AutoReplyScope::SpecificFriends | AutoReplyScope::FriendsExcept => {
            payload.uids.unwrap_or_default()
        }
        _ => Vec::new(),
    };

    let params = serde_json::json!({
        "cliLang": client.ctx().language,
        "id": payload.id,
        "enable": payload.is_enable,
        "content": payload.content,
        "startTime": payload.start_time,
        "endTime": payload.end_time,
        "recurrence": ["RRULE:FREQ=DAILY;"],
        "scope": payload.scope as i32,
        "uids": result_uids,
    });

    let url = client.service_url("auto_reply", "/api/autoreply/update")?;
    client.post_encrypted(&url, &params).await
}
