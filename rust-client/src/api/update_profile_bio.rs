use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProfileBioResponse(pub Value);

/// Update profile bio/status.
pub async fn update_profile_bio(
    client: &ApiClient,
    status: &str,
) -> Result<UpdateProfileBioResponse> {
    let params = serde_json::json!({ "status": status });

    let base_url = client.service_url("profile", "/api/social/profile/status")?;
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&base_url, &[("params", encrypted.clone())], true)?;
    let res = client
        .request_post(&url, &[("params", encrypted)])
        .await?;
    let val: Value = client.resolve(res).await?;
    Ok(UpdateProfileBioResponse(val))
}
