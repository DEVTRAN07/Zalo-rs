//! Get alias list.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AliasItem {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub alias: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAliasListResponse {
    #[serde(default)]
    pub items: Vec<AliasItem>,
    #[serde(rename = "updateTime")]
    #[serde(default)]
    pub update_time: String,
}

/// Get alias list.
pub async fn get_alias_list(
    ctx: &ContextSession,
    service_map: &Value,
    count: Option<i32>,
    page: Option<i32>,
) -> Result<GetAliasListResponse> {
    let base_url = service_url(service_map, "alias", "/api/alias/list")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "page": page.unwrap_or(1),
        "count": count.unwrap_or(100),
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
