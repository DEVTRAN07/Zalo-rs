use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveGroupBlockedMemberResponse {
    #[serde(rename = "errorMembers", default)]
    pub error_members: Vec<String>,
    #[serde(default)]
    pub error_data: Value,
}

/// Remove group blocked member.
pub async fn remove_group_blocked_member(
    client: &ApiClient,
    member_id: &str,
    group_id: &str,
) -> Result<RemoveGroupBlockedMemberResponse> {
    let base_url = client.service_url("group", "/api/group/blockedmems/remove")?;
    let params = serde_json::json!({
        "grid": group_id,
        "members": [member_id],
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
