//! Create a new group.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::client::ApiClient;
use crate::api::change_group_avatar::change_group_avatar;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupOptions {
    #[serde(default)]
    pub name: Option<String>,
    pub members: Vec<String>,
    /// Avatar source, can be a file path.
    #[serde(default)]
    pub avatar_source: Option<String>,
    /// Path to the avatar image file.
    #[serde(default)]
    pub avatar_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateGroupResponse {
    #[serde(rename = "groupType", default)]
    pub group_type: i64,
    #[serde(rename = "sucessMembers", default)]
    pub sucess_members: Vec<String>,
    #[serde(rename = "groupId", default)]
    pub group_id: String,
    #[serde(rename = "errorMembers", default)]
    pub error_members: Vec<String>,
    #[serde(default)]
    pub error_data: HashMap<String, serde_json::Value>,
}

/// Create a new group.
pub async fn create_group(
    client: &ApiClient,
    options: CreateGroupOptions,
) -> Result<CreateGroupResponse> {
    if options.members.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new(
            "Group must have at least one member",
        )));
    }

    let url = client.service_url("group", "/api/group/create/v2")?;
    let url = client.make_url(&url, &[], true)?;

    let client_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;
    let default_name = client_id.to_string();
    let (gname, name_changed) = match options.name.as_deref() {
        Some(n) if !n.is_empty() => (n.to_string(), 1),
        _ => (default_name, 0),
    };
    let member_types: Vec<i64> = options.members.iter().map(|_| -1).collect();

    let params = serde_json::json!({
        "clientId": client_id,
        "gname": gname,
        "gdesc": null,
        "members": options.members,
        "membersTypes": member_types,
        "nameChanged": name_changed,
        "createLink": 1,
        "clientLang": client.ctx().language,
        "imei": client.ctx().imei,
        "zsource": 601,
    });

    let data: CreateGroupResponse = client.post_encrypted(&url, &params).await?;

    let avatar_source = options.avatar_source.or(options.avatar_path);
    if let Some(avatar_path) = avatar_source {
        if let Err(e) = change_group_avatar(client, &avatar_path, &data.group_id).await {
            log::warn!("Failed to upload group avatar: {}", e);
        }
    }

    Ok(data)
}
