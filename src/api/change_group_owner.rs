//! Change group owner.

use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangeGroupOwnerResponse {
    #[serde(default)]
    pub time: i64,
}

/// Change group owner.
pub async fn change_group_owner(
    client: &ApiClient,
    member_id: &str,
    group_id: &str,
) -> Result<ChangeGroupOwnerResponse> {
    let url = client.service_url("group", "/api/group/change-owner")?;
    let params = serde_json::json!({
        "grid": group_id,
        "newAdminId": member_id,
        "imei": client.ctx().imei,
        "language": client.ctx().language,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
