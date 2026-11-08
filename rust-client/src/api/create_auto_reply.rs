//! Create an auto-reply rule (zBusiness).

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::auto_reply::AutoReplyItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAutoReplyPayload {
    pub content: String,
    #[serde(rename = "isEnable")]
    pub is_enable: bool,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    /// Scope value: 0 Everyone, 1 Stranger, 2 SpecificFriends, 3 FriendsExcept.
    pub scope: i64,
    #[serde(default)]
    pub uids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAutoReplyResponse {
    pub item: Option<AutoReplyItem>,
    #[serde(default)]
    pub version: i64,
}

/// Create an auto-reply rule.
pub async fn create_auto_reply(
    client: &ApiClient,
    payload: CreateAutoReplyPayload,
) -> Result<CreateAutoReplyResponse> {
    let url = client.service_url("auto_reply", "/api/autoreply/create")?;
    let url = client.make_url(&url, &[], true)?;

    let uids = payload.uids.unwrap_or_default();
    let result_uids = if payload.scope == 2 || payload.scope == 3 {
        uids
    } else {
        Vec::<String>::new()
    };

    let params = serde_json::json!({
        "cliLang": client.ctx().language,
        "enable": payload.is_enable,
        "content": payload.content,
        "startTime": payload.start_time,
        "endTime": payload.end_time,
        "recurrence": ["RRULE:FREQ=DAILY;"],
        "scope": payload.scope,
        "uids": result_uids,
    });

    client.post_encrypted(&url, &params).await
}
