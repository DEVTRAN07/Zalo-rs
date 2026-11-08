//! Enable and create a new group invite link.

use serde::{Deserialize, Serialize};

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableGroupLinkResponse {
    #[serde(default)]
    pub link: String,
    #[serde(default)]
    pub expiration_date: i64,
    #[serde(default)]
    pub enabled: i64,
}

/// Enable and create a new group link.
pub async fn enable_group_link(
    ctx: &ContextSession,
    service_map: &serde_json::Value,
    group_id: &str,
) -> Result<EnableGroupLinkResponse> {
    let base_url = service_url(service_map, "group", "/api/group/link/new")?;
    let utils = FactoryUtils::new(ctx);

    let params = serde_json::json!({
        "grid": group_id,
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = utils.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
