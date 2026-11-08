//! Disable a group invite link.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableGroupLinkResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Disable a group link.
pub async fn disable_group_link(
    client: &ApiClient,
    group_id: &str,
) -> Result<DisableGroupLinkResponse> {
    let url = client.service_url("group", "/api/group/link/disable")?;

    let params = serde_json::json!({ "grid": group_id });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = client.make_url(&url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
