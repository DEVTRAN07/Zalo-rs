//! Get group invite box info.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupInviteBoxInfoResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get group invite box info.
pub async fn get_group_invite_box_info(
    ctx: &ContextSession,
    service_map: &Value,
    group_id: &str,
) -> Result<GetGroupInviteBoxInfoResponse> {
    let base_url = service_url(service_map, "group", "/api/group/invite-box/info")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "grid": group_id,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
