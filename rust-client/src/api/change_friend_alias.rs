//! Change a friend's alias.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangeFriendAliasResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Change a friend's alias (nickname).
pub async fn change_friend_alias(
    ctx: &ContextSession,
    service_map: &Value,
    alias: &str,
    friend_id: &str,
) -> Result<ChangeFriendAliasResponse> {
    let base_url = service_url(service_map, "alias", "/api/alias/update")?;
    let utils = FactoryUtils::new(ctx);

    let params = serde_json::json!({
        "friendId": friend_id,
        "alias": alias,
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = utils.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
