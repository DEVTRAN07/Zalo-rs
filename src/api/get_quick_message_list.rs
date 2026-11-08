use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetQuickMessageListResponse {
    #[serde(default)]
    pub cursor: i64,
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub items: Vec<Value>,
}

/// Get quick message list.
pub async fn get_quick_message_list(
    client: &ApiClient,
) -> Result<GetQuickMessageListResponse> {
    let base_url = client.service_url("quick_message", "/api/quickmessage/list")?;
    let params = serde_json::json!({
        "version": 0,
        "lang": 0,
        "imei": client.ctx().imei,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = url_with_params(&base_url, &encrypted);
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
