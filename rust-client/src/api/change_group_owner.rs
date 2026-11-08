//! Change group owner.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangeGroupOwnerResponse {
    #[serde(default)]
    pub time: i64,
}

/// Change group owner.
pub async fn change_group_owner(
    ctx: &ContextSession,
    service_map: &Value,
    member_id: &str,
    group_id: &str,
) -> Result<ChangeGroupOwnerResponse> {
    let base_url = service_url(service_map, "group", "/api/group/change-owner")?;
    let utils = FactoryUtils::new(ctx);

    let params = serde_json::json!({
        "grid": group_id,
        "newAdminId": member_id,
        "imei": ctx.imei,
        "language": ctx.language,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = utils.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
