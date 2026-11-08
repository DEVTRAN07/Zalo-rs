//! Review pending member request.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

pub type ReviewPendingMemberRequestResponse = HashMap<String, i64>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewPendingMemberRequestPayload {
    pub members: Vec<String>,
    #[serde(rename = "isApprove")]
    pub is_approve: bool,
}

/// Review pending member(s) request.
pub async fn review_pending_member_request(
    ctx: &ContextSession,
    service_map: &Value,
    payload: ReviewPendingMemberRequestPayload,
    group_id: &str,
) -> Result<ReviewPendingMemberRequestResponse> {
    let base_url = service_url(service_map, "group", "/api/group/pending-mems/review")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "grid": group_id,
        "members": payload.members,
        "isApprove": if payload.is_approve { 1 } else { 0 },
    });
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
