//! Add user(s) to an existing group.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddUserToGroupResponse {
    #[serde(rename = "errorMembers", default)]
    pub error_members: Vec<String>,
    #[serde(default)]
    pub error_data: HashMap<String, serde_json::Value>,
}

/// Add user(s) to an existing group.
pub async fn add_user_to_group(
    ctx: &ContextSession,
    service_map: &serde_json::Value,
    member_ids: Vec<String>,
    group_id: &str,
) -> Result<AddUserToGroupResponse> {
    let base_url = service_url(service_map, "group", "/api/group/invite/v2")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let member_types: Vec<i64> = member_ids.iter().map(|_| -1).collect();
    let params = serde_json::json!({
        "grid": group_id,
        "members": member_ids,
        "memberTypes": member_types,
        "imei": ctx.imei,
        "clientLang": ctx.language,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
