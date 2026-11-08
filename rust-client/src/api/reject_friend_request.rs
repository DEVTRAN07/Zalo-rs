//! Reject a friend request.

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

pub type RejectFriendRequestResponse = Value;

/// Reject a friend request from a user.
pub async fn reject_friend_request(
    ctx: &ContextSession,
    service_map: &Value,
    friend_id: &str,
) -> Result<RejectFriendRequestResponse> {
    let base_url = service_url(service_map, "friend", "/api/friend/reject")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({ "fid": friend_id });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
