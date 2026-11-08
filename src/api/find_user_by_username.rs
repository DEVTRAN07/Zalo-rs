//! Find user by username.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::AvatarSize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FindUserByUsernameResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Find user by username.
pub async fn find_user_by_username(
    client: &ApiClient,
    username: &str,
    avatar_size: Option<AvatarSize>,
) -> Result<FindUserByUsernameResponse> {
    let avatar = avatar_size.unwrap_or(AvatarSize::Large) as i64;
    let url = client.service_url("friend", "/api/friend/search/by-user-name")?;

    let params = serde_json::json!({
        "user_name": username,
        "avatar_size": avatar,
    });

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = client.make_url(&url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
