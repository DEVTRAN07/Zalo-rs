//! Add group deputy/deputies.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddGroupDeputyResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Add group deputy/deputies.
pub async fn add_group_deputy(
    client: &ApiClient,
    member_ids: Vec<String>,
    group_id: &str,
) -> Result<AddGroupDeputyResponse> {
    let url = client.service_url("group", "/api/group/admins/add")?;
    let params = serde_json::json!({
        "grid": group_id,
        "members": member_ids,
        "imei": client.ctx().imei,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
