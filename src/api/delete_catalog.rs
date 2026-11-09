//! Delete a catalog.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteCatalogResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Delete a catalog.
pub async fn delete_catalog(
    client: &ApiClient,
    catalog_id: &str,
) -> Result<DeleteCatalogResponse> {
    let url = client.service_url("catalog", "/api/prodcatalog/catalog/delete")?;
    let url = client.make_url(&url, &[], true)?;

    let params = serde_json::json!({ "catalog_id": catalog_id });
    client.post_encrypted(&url, &params).await
}
