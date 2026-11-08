//! Lock a poll.

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

pub type LockPollResponse = Value;

/// Lock a poll, preventing further votes.
pub async fn lock_poll(
    ctx: &ContextSession,
    service_map: &Value,
    poll_id: i64,
) -> Result<LockPollResponse> {
    let base_url = service_url(service_map, "group", "/api/poll/end")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "poll_id": poll_id,
        "imei": ctx.imei,
    });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
