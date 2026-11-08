//! Add user(s) to an existing group.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddUserToGroupResponse {
    #[serde(rename = "errorMembers", default)]
    pub error_members: Vec<String>,
    #[serde(default)]
    pub error_data: HashMap<String, serde_json::Value>,
}

/// Add user(s) to an existing group.
pub async fn add_user_to_group(
    client: &ApiClient,
    member_ids: Vec<String>,
    group_id: &str,
) -> Result<AddUserToGroupResponse> {
    let url = client.service_url("group", "/api/group/invite/v2")?;
    let url = client.make_url(&url, &[], true)?;

    let member_types: Vec<i64> = member_ids.iter().map(|_| -1).collect();
    let params = serde_json::json!({
        "grid": group_id,
        "members": member_ids,
        "memberTypes": member_types,
        "imei": client.ctx().imei,
        "clientLang": client.ctx().language,
    });

    client.post_encrypted(&url, &params).await
}
