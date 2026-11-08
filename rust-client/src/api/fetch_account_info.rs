//! Fetch current account profile info.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::Result;
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FetchAccountInfoResponse {
    #[serde(default)]
    pub profile: Value,
}

/// Fetch the current account's profile.
pub async fn fetch_account_info(
    ctx: &ContextSession,
    service_map: &Value,
) -> Result<FetchAccountInfoResponse> {
    let base_url = service_url(service_map, "profile", "/api/social/profile/me-v2")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;
    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
