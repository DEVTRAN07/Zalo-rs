//! Create an auto-reply rule (zBusiness).

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
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
    ctx: &ContextSession,
    service_map: &Value,
    payload: CreateAutoReplyPayload,
) -> Result<CreateAutoReplyResponse> {
    let base_url = service_url(service_map, "auto_reply", "/api/autoreply/create")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let uids = payload.uids.unwrap_or_default();
    let result_uids = if payload.scope == 2 || payload.scope == 3 {
        uids
    } else {
        Vec::<String>::new()
    };

    let params = serde_json::json!({
        "cliLang": ctx.language,
        "enable": payload.is_enable,
        "content": payload.content,
        "startTime": payload.start_time,
        "endTime": payload.end_time,
        "recurrence": ["RRULE:FREQ=DAILY;"],
        "scope": payload.scope,
        "uids": result_uids,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
