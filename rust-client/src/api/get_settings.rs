//! Get account settings.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSettingsResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get my account settings.
pub async fn get_settings(ctx: &ContextSession, _service_map: &Value) -> Result<GetSettingsResponse> {
    let base_url = "https://wpa.chat.zalo.me/api/setting/me";
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(base_url, &[], true)?;

    let params = serde_json::json!({});
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
