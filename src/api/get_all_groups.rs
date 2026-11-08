//! Get all groups.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAllGroupsResponse {
    #[serde(default)]
    pub version: String,
    #[serde(rename = "gridVerMap")]
    #[serde(default)]
    pub grid_ver_map: HashMap<String, String>,
}

/// Get all groups.
pub async fn get_all_groups(client: &ApiClient) -> Result<GetAllGroupsResponse> {
    let url = client.service_url("group_poll", "/api/group/getlg/v4")?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
