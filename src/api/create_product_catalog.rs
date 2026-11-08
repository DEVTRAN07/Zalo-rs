//! Create a product catalog item (zBusiness).

use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::product_catalog::ProductCatalogItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductCatalogPayload {
    #[serde(rename = "catalogId")]
    pub catalog_id: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    pub price: String,
    pub description: String,
    /// Pre-uploaded photo URLs (up to 5). File upload is not implemented.
    #[serde(rename = "product_photos", default)]
    pub product_photos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateProductCatalogResponse {
    pub item: Option<ProductCatalogItem>,
    #[serde(default)]
    pub version_ls_catalog: i64,
    #[serde(default)]
    pub version_catalog: i64,
}

/// Create a product catalog item.
pub async fn create_product_catalog(
    client: &ApiClient,
    payload: CreateProductCatalogPayload,
) -> Result<CreateProductCatalogResponse> {
    if payload.product_photos.len() > 5 {
        return Err(ZaloError::Api(ZaloApiError::new(
            "Maximum 5 media files are allowed",
        )));
    }

    let url = client.service_url("catalog", "/api/prodcatalog/product/create")?;
    let url = client.make_url(&url, &[], true)?;

    let create_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let params = serde_json::json!({
        "product_name": payload.product_name,
        "price": payload.price,
        "description": payload.description,
        "product_photos": payload.product_photos,
        "catalog_id": payload.catalog_id,
        "currency_unit": "₫",
        "create_time": create_time,
    });

    client.post_encrypted(&url, &params).await
}
