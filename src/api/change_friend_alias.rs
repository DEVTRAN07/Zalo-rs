//! Change a friend's alias.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangeFriendAliasResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Change a friend's alias (nickname).
pub async fn change_friend_alias(
    client: &ApiClient,
    alias: &str,
    friend_id: &str,
) -> Result<ChangeFriendAliasResponse> {
    let url = client.service_url("alias", "/api/alias/update")?;
    let params = serde_json::json!({
        "friendId": friend_id,
        "alias": alias,
        "imei": client.ctx().imei,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
