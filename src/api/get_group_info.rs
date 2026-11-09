//! Get group information.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::Result;
use crate::models::group::GroupInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfoPendingApprove {
    pub time: i64,
    pub uids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfoWithExtra {
    #[serde(flatten)]
    pub info: GroupInfo,
    #[serde(rename = "memVerList")]
    #[serde(default)]
    pub mem_ver_list: Vec<String>,
    #[serde(rename = "pendingApprove")]
    #[serde(default)]
    pub pending_approve: Option<GroupInfoPendingApprove>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupInfoResponse {
    #[serde(rename = "removedsGroup")]
    #[serde(default)]
    pub removeds_group: Vec<String>,
    #[serde(rename = "unchangedsGroup")]
    #[serde(default)]
    pub unchangeds_group: Vec<String>,
    #[serde(rename = "gridInfoMap")]
    #[serde(default)]
    pub grid_info_map: HashMap<String, GroupInfoWithExtra>,
}

/// Get group information.
pub async fn get_group_info(
    client: &ApiClient,
    group_ids: Vec<String>,
) -> Result<GetGroupInfoResponse> {
    let url = client.service_url("group", "/api/group/getmg-v2")?;
    let mut grid_ver_map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    for id in group_ids {
        grid_ver_map.insert(id, 0);
    }

    let params = serde_json::json!({
        "gridVerMap": serde_json::to_string(&grid_ver_map).unwrap_or_default(),
    });

    client.post_encrypted(&url, &params).await
}
