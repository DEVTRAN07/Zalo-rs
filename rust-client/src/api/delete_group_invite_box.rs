//! Delete group invite box entries.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

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
    ctx: &ContextSession,
    service_map: &serde_json::Value,
    group_ids: Vec<String>,
    block_future_invite: bool,
) -> Result<DeleteGroupInviteBoxResponse> {
    let base_url = service_url(service_map, "group", "/api/group/inv-box/mdel-inv")?;
    let utils = FactoryUtils::new(ctx);

    let invitations: Vec<serde_json::Value> = group_ids
        .into_iter()
        .map(|grid| serde_json::json!({ "grid": grid }))
        .collect();

    let params = serde_json::json!({
        "invitations": serde_json::to_string(&invitations).unwrap_or_else(|_| "[]".to_string()),
        "block": if block_future_invite { 1 } else { 0 },
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = utils.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
