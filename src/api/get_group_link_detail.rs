//! Get group link detail.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupLinkDetailResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get group link detail.
pub async fn get_group_link_detail(
    client: &ApiClient,
    link: &str,
) -> Result<GetGroupLinkDetailResponse> {
    let url = client.service_url("group", "/api/group/link/getdetail")?;
    let params = serde_json::json!({
        "link": link,
    });

    client.post_encrypted(&url, &params).await
}
