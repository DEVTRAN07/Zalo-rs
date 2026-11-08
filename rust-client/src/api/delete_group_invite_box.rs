//! Delete group invite box entries.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InviteBoxError {
    #[serde(default)]
    pub err: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteGroupInviteBoxResponse {
    #[serde(rename = "delInvitaionIds", default)]
    pub del_invitaion_ids: Vec<String>,
    #[serde(rename = "errMap", default)]
    pub err_map: HashMap<String, InviteBoxError>,
}

/// Delete group invite box entries.
pub async fn delete_group_invite_box(
    client: &ApiClient,
    group_ids: Vec<String>,
    block_future_invite: bool,
) -> Result<DeleteGroupInviteBoxResponse> {
    let url = client.service_url("group", "/api/group/inv-box/mdel-inv")?;

    let invitations: Vec<serde_json::Value> = group_ids
        .into_iter()
        .map(|grid| serde_json::json!({ "grid": grid }))
        .collect();

    let params = serde_json::json!({
        "invitations": serde_json::to_string(&invitations).unwrap_or_else(|_| "[]".to_string()),
        "block": if block_future_invite { 1 } else { 0 },
    });

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = client.make_url(&url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
