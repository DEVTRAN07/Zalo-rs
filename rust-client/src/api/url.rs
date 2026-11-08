//! Service URL resolution from `zpw_service_map`.

use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

/// Resolve a service URL from the zpw_service_map.
pub fn service_url(service_map: &Value, service: &str, path: &str) -> Result<String> {
    let base = service_map
        .get(service)
        .and_then(|v| v.as_array())
        .and_then(|a| a.first())
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            ZaloError::Api(ZaloApiError::new(format!(
                "Missing service URL for '{}' in zpw_service_map",
                service
            )))
        })?;
    let base = base.trim_end_matches('/');
    let path = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    };
    Ok(format!("{}{}", base, path))
}
