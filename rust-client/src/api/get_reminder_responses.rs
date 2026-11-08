//! Get reminder responses.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetReminderResponsesResponse {
    #[serde(rename = "rejectMember", default)]
    pub reject_member: Vec<String>,
    #[serde(rename = "acceptMember", default)]
    pub accept_member: Vec<String>,
}

/// Get reminder responses.
pub async fn get_reminder_responses(
    ctx: &ContextSession,
    service_map: &Value,
    reminder_id: &str,
) -> Result<GetReminderResponsesResponse> {
    let base_url = service_url(service_map, "group_board", "/api/board/topic/listResponseEvent")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({ "eventId": reminder_id });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_get(&url_with_params(&url, &encrypted)).await?;
    utils.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
