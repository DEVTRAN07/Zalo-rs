use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::catalog::CatalogItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCatalogPayload {
    #[serde(rename = "catalogId")]
    pub catalog_id: String,
    #[serde(rename = "catalogName")]
    pub catalog_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCatalogResponse {
    #[serde(default)]
    pub item: Option<CatalogItem>,
    #[serde(rename = "version_ls_catalog", default)]
    pub version_ls_catalog: i64,
    #[serde(rename = "version_catalog", default)]
    pub version_catalog: i64,
}

/// Update catalog (zBusiness).
pub async fn update_catalog(
    client: &ApiClient,
    payload: UpdateCatalogPayload,
) -> Result<UpdateCatalogResponse> {
    let params = serde_json::json!({
        "catalog_id": payload.catalog_id,
        "catalog_name": payload.catalog_name,
        "catalog_photo": "",
    });

    let url = client.service_url("catalog", "/api/prodcatalog/catalog/update")?;
    client.post_encrypted(&url, &params).await
}
