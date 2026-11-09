use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSettingsResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get my account settings.
pub async fn get_settings(client: &ApiClient) -> Result<GetSettingsResponse> {
    let base_url = "https://wpa.chat.zalo.me/api/setting/me";
    let url = client.make_url(base_url, &[], true)?;
    let params = serde_json::json!({});
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = if url.contains('?') { format!("{}&params={}", &url, urlencoding::encode(&encrypted)) } else { format!("{}?params={}", &url, urlencoding::encode(&encrypted)) };
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
