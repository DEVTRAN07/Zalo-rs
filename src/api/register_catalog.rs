use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegisterCatalogResponse {
    #[serde(default)]
    pub status: i64,
    #[serde(rename = "prodCatalog", default)]
    pub prod_catalog: Option<Value>,
}

/// Register catalog (zBusiness).
pub async fn register_catalog(client: &ApiClient, name: &str) -> Result<RegisterCatalogResponse> {
    let base_url = client.service_url("catalog", "/api/prodcatalog/product/create")?;
    let url = client.make_url(&base_url, &[], true)?;
    let params = serde_json::json!({
        "catalog_name": name,
        "catalog_list": serde_json::to_string(&[-1]).unwrap_or_else(|_| "[-1]".to_string()),
    });
    client.post_encrypted(&url, &params).await
}
