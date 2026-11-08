//! Disperse (delete) a group.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisperseGroupResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Disperse a group.
pub async fn disperse_group(
    client: &ApiClient,
    group_id: &str,
) -> Result<DisperseGroupResponse> {
    let url = client.service_url("group", "/api/group/disperse")?;
    let url = client.make_url(&url, &[], true)?;

    let params = serde_json::json!({
        "grid": group_id,
        "imei": client.ctx().imei,
    });

    client.post_encrypted(&url, &params).await
}
