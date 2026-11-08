//! Remove friend's alias.

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

pub type RemoveFriendAliasResponse = Value;

/// Remove friend's alias.
pub async fn remove_friend_alias(
    ctx: &ContextSession,
    service_map: &Value,
    friend_id: &str,
) -> Result<RemoveFriendAliasResponse> {
    let base_url = service_url(service_map, "alias", "/api/alias/remove")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({ "friendId": friend_id });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_get(&url_with_params(&url, &encrypted)).await?;
    utils.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
