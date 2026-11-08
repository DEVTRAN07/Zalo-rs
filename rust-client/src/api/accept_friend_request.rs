//! Accept a friend request.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AcceptFriendRequestResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Accept a friend request from a user.
pub async fn accept_friend_request(
    ctx: &ContextSession,
    service_map: &Value,
    friend_id: &str,
) -> Result<AcceptFriendRequestResponse> {
    let base_url = service_url(service_map, "friend", "/api/friend/accept")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "fid": friend_id,
        "language": ctx.language,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
