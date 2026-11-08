//! Get all groups.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::Result;
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAllGroupsResponse {
    #[serde(default)]
    pub version: String,
    #[serde(rename = "gridVerMap")]
    #[serde(default)]
    pub grid_ver_map: HashMap<String, String>,
}

/// Get all groups.
pub async fn get_all_groups(ctx: &ContextSession, service_map: &Value) -> Result<GetAllGroupsResponse> {
    let base_url = service_url(service_map, "group_poll", "/api/group/getlg/v4")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
