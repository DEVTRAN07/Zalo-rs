//! Create a catalog (zBusiness).

use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::catalog::CatalogItem;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCatalogResponse {
    pub item: Option<CatalogItem>,
    #[serde(default)]
    pub version_ls_catalog: i64,
    #[serde(default)]
    pub version_catalog: i64,
}

/// Create a catalog.
pub async fn create_catalog(
    client: &ApiClient,
    catalog_name: &str,
) -> Result<CreateCatalogResponse> {
    let url = client.service_url("catalog", "/api/prodcatalog/catalog/create")?;
    let url = client.make_url(&url, &[], true)?;

    let params = serde_json::json!({
        "catalog_name": catalog_name,
        "catalog_photo": "",
    });

    client.post_encrypted(&url, &params).await
}
