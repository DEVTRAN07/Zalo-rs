//! Leave group.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LeaveGroupResponse {
    #[serde(rename = "memberError", default)]
    pub member_error: Vec<Value>,
}

/// Leave group.
pub async fn leave_group(
    ctx: &ContextSession,
    service_map: &Value,
    group_id: &str,
    silent: bool,
) -> Result<LeaveGroupResponse> {
    let base_url = service_url(service_map, "group", "/api/group/leave")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "grids": [group_id],
        "imei": ctx.imei,
        "silent": if silent { 1 } else { 0 },
        "language": ctx.language,
    });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
