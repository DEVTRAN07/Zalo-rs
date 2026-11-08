//! Create a catalog (zBusiness).

use serde::{Deserialize, Serialize};

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
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
    _ctx: &ContextSession,
    service_map: &serde_json::Value,
    catalog_name: &str,
) -> Result<CreateCatalogResponse> {
    let base_url = service_url(service_map, "catalog", "/api/prodcatalog/catalog/create")?;
    let utils = FactoryUtils::new(_ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "catalog_name": catalog_name,
        "catalog_photo": "",
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
