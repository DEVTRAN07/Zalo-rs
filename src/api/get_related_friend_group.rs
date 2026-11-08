use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRelatedFriendGroupResponse {
    #[serde(rename = "groupRelateds", default)]
    pub group_relateds: HashMap<String, Vec<String>>,
}

/// Get related friend group (zBusiness).
pub async fn get_related_friend_group(
    client: &ApiClient,
    friend_ids: &[String],
) -> Result<GetRelatedFriendGroupResponse> {
    let url = client.service_url("friend", "/api/friend/group/related")?;
    let params = serde_json::json!({
        "friend_ids": serde_json::to_string(friend_ids).unwrap_or_else(|_| "[]".to_string()),
        "imei": client.ctx().imei,
    });
    client.post_encrypted(&url, &params).await
}
