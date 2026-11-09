use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GridMessage {
    #[serde(default)]
    pub error_code: i64,
    #[serde(default)]
    pub error_message: String,
    #[serde(default)]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InviteUserToGroupsResponse {
    #[serde(flatten, default)]
    pub grid_message_map: std::collections::HashMap<String, GridMessage>,
}

/// Invite a user to groups (multiple).
pub async fn invite_user_to_groups(
    client: &ApiClient,
    user_id: &str,
    group_ids: &[String],
) -> Result<InviteUserToGroupsResponse> {
    let base_url = client.service_url("group", "/api/group/invite/multi")?;
    let params = serde_json::json!({
        "grids": serde_json::to_string(group_ids).unwrap_or_else(|_| "[]".to_string()),
        "member": serde_json::to_string(&[user_id]).unwrap_or_else(|_| "[]".to_string()),
        "memberTypes": serde_json::to_string(&vec![-1i32; group_ids.len()]).unwrap_or_else(|_| "[]".to_string()),
        "srcInteraction": 2,
        "clientLang": client.ctx().language,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = if base_url.contains('?') { format!("{}&params={}", &base_url, urlencoding::encode(&encrypted)) } else { format!("{}?params={}", &base_url, urlencoding::encode(&encrypted)) };
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
