//! Block/unblock a user from viewing your feed.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockViewFeedResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Block/unblock friend view feed by ID.
pub async fn block_view_feed(
    ctx: &ContextSession,
    service_map: &Value,
    is_block_feed: bool,
    user_id: &str,
) -> Result<BlockViewFeedResponse> {
    let base_url = service_url(service_map, "friend", "/api/friend/feed/block")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "fid": user_id,
        "isBlockFeed": if is_block_feed { 1 } else { 0 },
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
