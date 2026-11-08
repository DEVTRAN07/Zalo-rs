//! Enable and create a new group invite link.

use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableGroupLinkResponse {
    #[serde(default)]
    pub link: String,
    #[serde(default)]
    pub expiration_date: i64,
    #[serde(default)]
    pub enabled: i64,
}

/// Enable and create a new group link.
pub async fn enable_group_link(
    client: &ApiClient,
    group_id: &str,
) -> Result<EnableGroupLinkResponse> {
    let url = client.service_url("group", "/api/group/link/new")?;

    let params = serde_json::json!({
        "grid": group_id,
        "imei": client.ctx().imei,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = client.make_url(&url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
