//! Get group link info.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupLinkInfoResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get group link info.
pub async fn get_group_link_info(
    client: &ApiClient,
    group_id: &str,
) -> Result<GetGroupLinkInfoResponse> {
    let url = client.service_url("group", "/api/group/link/info")?;
    let params = serde_json::json!({
        "grid": group_id,
    });

    client.post_encrypted(&url, &params).await
}
