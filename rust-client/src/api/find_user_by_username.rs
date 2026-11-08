//! Find user by username.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::AvatarSize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FindUserByUsernameResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Find user by username.
pub async fn find_user_by_username(
    ctx: &ContextSession,
    service_map: &Value,
    username: &str,
    avatar_size: Option<AvatarSize>,
) -> Result<FindUserByUsernameResponse> {
    let avatar = avatar_size.unwrap_or(AvatarSize::Large) as i64;
    let base_url = service_url(service_map, "friend", "/api/friend/search/by-user-name")?;
    let utils = FactoryUtils::new(ctx);

    let params = serde_json::json!({
        "user_name": username,
        "avatar_size": avatar,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = utils.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
