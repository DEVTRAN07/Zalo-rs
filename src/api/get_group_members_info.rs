//! Get group members info.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupMembersInfoResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get group members info.
pub async fn get_group_members_info(
    client: &ApiClient,
    group_id: &str,
    member_ids: Vec<String>,
) -> Result<GetGroupMembersInfoResponse> {
    let url = client.service_url("group", "/api/group/getmembersinfo")?;
    let params = serde_json::json!({
        "grid": group_id,
        "mems": member_ids,
    });

    client.post_encrypted(&url, &params).await
}
