//! Change a group's name.

use serde::{Deserialize, Serialize};

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangeGroupNameResponse {
    #[serde(default)]
    pub status: i64,
}

/// Change group name.
pub async fn change_group_name(
    ctx: &ContextSession,
    service_map: &serde_json::Value,
    name: &str,
    group_id: &str,
) -> Result<ChangeGroupNameResponse> {
    let base_url = service_url(service_map, "group", "/api/group/updateinfo")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let gname = if name.is_empty() {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .to_string()
    } else {
        name.to_string()
    };

    let params = serde_json::json!({
        "grid": group_id,
        "gname": gname,
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
