//! Delete product catalog items.

use serde::{Deserialize, Serialize};

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProductCatalogPayload {
    #[serde(rename = "productIds")]
    pub product_ids: Vec<String>,
    #[serde(rename = "catalogId")]
    pub catalog_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteProductCatalogResponse {
    #[serde(default)]
    pub item: Vec<i64>,
    #[serde(default)]
    pub version_ls_catalog: i64,
    #[serde(default)]
    pub version_catalog: i64,
}

/// Delete product catalog items.
pub async fn delete_product_catalog(
    ctx: &ContextSession,
    service_map: &serde_json::Value,
    payload: DeleteProductCatalogPayload,
) -> Result<DeleteProductCatalogResponse> {
    let base_url = service_url(service_map, "catalog", "/api/prodcatalog/product/mdelete")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "product_ids": payload.product_ids,
        "catalog_id": payload.catalog_id,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
