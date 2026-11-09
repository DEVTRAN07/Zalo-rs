//! Get product catalog list.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};

use crate::errors::Result;
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
    client: &ApiClient,
    catalog_id: &str,
) -> Result<GetProductCatalogListResponse> {
    let url = client.service_url("catalog", "/api/prodcatalog/product/list")?;
    let params = serde_json::json!({
        "catalog_id": catalog_id,
    });

    client.post_encrypted(&url, &params).await
}
