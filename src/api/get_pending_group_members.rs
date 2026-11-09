//! Get pending group members.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetPendingGroupMembersResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get pending group members.
pub async fn get_pending_group_members(
    client: &ApiClient,
    group_id: &str,
) -> Result<GetPendingGroupMembersResponse> {
    let url = client.service_url("group", "/api/group/pending-list")?;
    let params = serde_json::json!({
        "grid": group_id,
    });

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let full_url = if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(&encrypted))
    } else {
        format!("{}?params={}", url, urlencoding::encode(&encrypted))
    };
    let res = client.request_get(&full_url).await?;
    client.resolve(res).await
}
