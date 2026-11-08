use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type ScanUrlResponse = Value;

/// Scan URL.
pub async fn scan_url(client: &ApiClient, url_to_scan: &str) -> Result<ScanUrlResponse> {
    let base_url = client.service_url("scan_url", "/api/scan-url")?;
    let url = client.make_url(&base_url, &[], true)?;
    let params = serde_json::json!({ "url": url_to_scan });
    client.post_encrypted(&url, &params).await
}
