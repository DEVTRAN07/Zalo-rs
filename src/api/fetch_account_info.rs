//! Fetch current account profile info.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FetchAccountInfoResponse {
    #[serde(default)]
    pub profile: Value,
}

/// Fetch the current account's profile.
pub async fn fetch_account_info(client: &ApiClient) -> Result<FetchAccountInfoResponse> {
    let url = client.service_url("profile", "/api/social/profile/me-v2")?;
    let url = client.make_url(&url, &[], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
