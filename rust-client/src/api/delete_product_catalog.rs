//! Delete product catalog items.

use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

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
    client: &ApiClient,
    payload: DeleteProductCatalogPayload,
) -> Result<DeleteProductCatalogResponse> {
    let url = client.service_url("catalog", "/api/prodcatalog/product/mdelete")?;
    let url = client.make_url(&url, &[], true)?;

    let params = serde_json::json!({
        "product_ids": payload.product_ids,
        "catalog_id": payload.catalog_id,
    });

    client.post_encrypted(&url, &params).await
}
