//! Get catalog list.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::catalog::CatalogItem;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetCatalogListPayload {
    pub limit: Option<i32>,
    #[serde(rename = "lastProductId")]
    pub last_product_id: Option<i64>,
    pub page: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetCatalogListResponse {
    #[serde(default)]
    pub items: Vec<CatalogItem>,
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub has_more: i32,
}

/// Get catalog list.
pub async fn get_catalog_list(
    ctx: &ContextSession,
    service_map: &Value,
    payload: Option<GetCatalogListPayload>,
) -> Result<GetCatalogListResponse> {
    let base_url = service_url(service_map, "catalog", "/api/prodcatalog/catalog/list")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let payload = payload.unwrap_or_default();
    let params = serde_json::json!({
        "version_list_catalog": 0,
        "limit": payload.limit.unwrap_or(20),
        "last_product_id": payload.last_product_id.unwrap_or(-1),
        "page": payload.page.unwrap_or(0),
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils
        .request_post(&url, &[("params", encrypted)])
        .await?;
    utils.resolve(res).await
}
