use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LeaveGroupResponse {
    #[serde(rename = "memberError", default)]
    pub member_error: Vec<Value>,
}

/// Leave group.
pub async fn leave_group(
    client: &ApiClient,
    group_id: &str,
    silent: Option<bool>,
) -> Result<LeaveGroupResponse> {
    let base_url = client.service_url("group", "/api/group/leave")?;
    let url = client.make_url(&base_url, &[], true)?;
    let params = serde_json::json!({
        "grids": [group_id],
        "imei": client.ctx().imei,
        "silent": if silent.unwrap_or(false) { 1 } else { 0 },
        "language": client.ctx().language,
    });
    client.post_encrypted(&url, &params).await
}
