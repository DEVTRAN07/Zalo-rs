use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProductCatalogPayload {
    #[serde(rename = "catalogId")]
    pub catalog_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    pub price: String,
    pub description: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// Product photo URLs (up to 5). The TS version can also upload files; in Rust,
    /// pass pre-uploaded URLs here (use `upload_product_photo` first if needed).
    #[serde(default)]
    pub product_photos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProductCatalogResponse {
    #[serde(default)]
    pub item: Value,
    #[serde(default)]
    pub version_ls_catalog: i64,
    #[serde(default)]
    pub version_catalog: i64,
}

/// Update a product in a catalog (zBusiness).
pub async fn update_product_catalog(
    client: &ApiClient,
    payload: UpdateProductCatalogPayload,
) -> Result<UpdateProductCatalogResponse> {
    if payload.product_photos.len() > 5 {
        return Err(ZaloError::Api(ZaloApiError::new(
            "Maximum 5 media files are allowed",
        )));
    }

    let params = serde_json::json!({
        "product_id": payload.product_id,
        "product_name": payload.product_name,
        "price": payload.price,
        "description": payload.description,
        "product_photos": payload.product_photos,
        "catalog_id": payload.catalog_id,
        "currency_unit": "₫",
        "create_time": payload.create_time,
    });

    let base_url = client.service_url("catalog", "/api/prodcatalog/product/update")?;
    let url = client.make_url(&base_url, &[], true)?;
    client.post_encrypted(&url, &params).await
}
