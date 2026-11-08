//! Register catalog.

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

pub type RegisterCatalogResponse = Value;

/// Register catalog.
pub async fn register_catalog(
    ctx: &ContextSession,
    service_map: &Value,
    enable: bool,
) -> Result<RegisterCatalogResponse> {
    let base_url = service_url(service_map, "catalog", "/api/prodcatalog/catalog/register")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({ "enable": if enable { 1 } else { 0 } });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
