use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewPendingMemberRequestType {
    Deny = 0,
    Accept = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReviewPendingMemberRequestResponse {
    #[serde(default)]
    pub data: Option<HashMap<String, Vec<Value>>>,
}

/// Review pending member(s) request.
pub async fn review_pending_member_request(
    client: &ApiClient,
    payload: (ReviewPendingMemberRequestType, &[String], &str),
) -> Result<ReviewPendingMemberRequestResponse> {
    let (req_type, members, group_id) = payload;
    let base_url = client.service_url("group", "/api/group/pending-mems/review")?;
    let params = serde_json::json!({
        "grid": group_id,
        "members": serde_json::to_string(members).unwrap_or_else(|_| "[]".to_string()),
        "isApprove": req_type as i32,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = url_with_params(&base_url, &encrypted);
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
