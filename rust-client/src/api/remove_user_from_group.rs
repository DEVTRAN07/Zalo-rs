//! Remove user from group.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveUserFromGroupResponse {
    #[serde(rename = "errorMembers", default)]
    pub error_members: Vec<String>,
}

/// Remove user(s) from an existing group.
pub async fn remove_user_from_group(
    ctx: &ContextSession,
    service_map: &Value,
    member_ids: &[String],
    group_id: &str,
) -> Result<RemoveUserFromGroupResponse> {
    let base_url = service_url(service_map, "group", "/api/group/kickout")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "grid": group_id,
        "members": member_ids,
        "imei": ctx.imei,
    });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
