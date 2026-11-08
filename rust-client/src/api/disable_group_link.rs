//! Disable a group invite link.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableGroupLinkResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Disable a group link.
pub async fn disable_group_link(
    ctx: &ContextSession,
    service_map: &Value,
    group_id: &str,
) -> Result<DisableGroupLinkResponse> {
    let base_url = service_url(service_map, "group", "/api/group/link/disable")?;
    let utils = FactoryUtils::new(ctx);

    let params = serde_json::json!({ "grid": group_id });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = utils.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
