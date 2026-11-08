//! Get product catalog list.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::product_catalog::ProductCatalogItem;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetProductCatalogListResponse {
    #[serde(default)]
    pub items: Vec<ProductCatalogItem>,
    #[serde(default)]
    pub version: i64,
}

/// Get product catalog list.
pub async fn get_product_catalog_list(
    ctx: &ContextSession,
    service_map: &Value,
    catalog_id: &str,
) -> Result<GetProductCatalogListResponse> {
    let base_url = service_url(service_map, "catalog", "/api/prodcatalog/product/list")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "catalog_id": catalog_id,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
