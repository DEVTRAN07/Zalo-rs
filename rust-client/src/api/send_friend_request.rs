//! Send a friend request.

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

pub type SendFriendRequestResponse = Value;

/// Send a friend request to a user.
pub async fn send_friend_request(
    ctx: &ContextSession,
    service_map: &Value,
    msg: &str,
    user_id: &str,
) -> Result<SendFriendRequestResponse> {
    let base_url = service_url(service_map, "friend", "/api/friend/sendreq")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "toid": user_id,
        "msg": msg,
        "reqsrc": 30,
        "imei": ctx.imei,
        "language": ctx.language,
        "srcParams": serde_json::json!({ "uidTo": user_id }).to_string(),
    });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
