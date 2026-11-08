use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpgradeGroupToCommunityResponse(pub Value);

/// Upgrade a group to a community.
/// Note: Zalo account identity must be verified and the account must be over 18.
pub async fn upgrade_group_to_community(
    client: &ApiClient,
    group_id: &str,
) -> Result<UpgradeGroupToCommunityResponse> {
    let params = serde_json::json!({
        "grId": group_id,
        "language": client.ctx().language,
    });

    let base_url = client.service_url("group", "/api/group/upgrade/community")?;
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    let val: Value = client.resolve(res).await?;
    Ok(UpgradeGroupToCommunityResponse(val))
}
