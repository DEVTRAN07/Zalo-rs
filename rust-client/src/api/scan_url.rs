//! Scan URL.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanUrlResponse {
    #[serde(rename = "isSafe", default)]
    pub is_safe: bool,
}

/// Scan URL to check if it is safe.
pub async fn scan_url(
    ctx: &ContextSession,
    service_map: &Value,
    url_to_scan: &str,
) -> Result<ScanUrlResponse> {
    let base_url = service_url(service_map, "file", "/api/message/scanurl")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({ "url": url_to_scan });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
