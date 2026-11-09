//! Get catalog list.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};

use crate::errors::Result;
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
    client: &ApiClient,
    payload: Option<GetCatalogListPayload>,
) -> Result<GetCatalogListResponse> {
    let url = client.service_url("catalog", "/api/prodcatalog/catalog/list")?;
    let payload = payload.unwrap_or_default();
    let params = serde_json::json!({
        "version_list_catalog": 0,
        "limit": payload.limit.unwrap_or(20),
        "last_product_id": payload.last_product_id.unwrap_or(-1),
        "page": payload.page.unwrap_or(0),
    });

    client.post_encrypted(&url, &params).await
}
