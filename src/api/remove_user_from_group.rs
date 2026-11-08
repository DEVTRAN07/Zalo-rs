use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveUserFromGroupResponse {
    #[serde(rename = "errorMembers", default)]
    pub error_members: Vec<String>,
}

/// Remove user(s) from an existing group.
pub async fn remove_user_from_group(
    client: &ApiClient,
    member_ids: &[String],
    group_id: &str,
) -> Result<RemoveUserFromGroupResponse> {
    let url = client.service_url("group", "/api/group/kickout")?;
    let params = serde_json::json!({
        "grid": group_id,
        "members": member_ids,
        "imei": client.ctx().imei,
    });
    client.post_encrypted(&url, &params).await
}
