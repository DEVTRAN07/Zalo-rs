//! Delete an auto-reply rule.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteAutoReplyResponse {
    #[serde(default)]
    pub item: i64,
    #[serde(default)]
    pub version: i64,
}

/// Delete an auto-reply rule.
pub async fn delete_auto_reply(
    ctx: &ContextSession,
    service_map: &Value,
    id: i64,
) -> Result<DeleteAutoReplyResponse> {
    let base_url = service_url(service_map, "auto_reply", "/api/autoreply/delete")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "cliLang": ctx.language,
        "id": id,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
