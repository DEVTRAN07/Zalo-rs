//! Get quick message list.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

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
    ctx: &ContextSession,
    service_map: &Value,
) -> Result<GetQuickMessageListResponse> {
    let base_url = service_url(service_map, "quick_message", "/api/quickmessage/list")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "version": 0,
        "lang": 0,
        "imei": ctx.imei,
    });
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
