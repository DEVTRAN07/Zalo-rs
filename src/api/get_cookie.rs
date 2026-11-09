//! Get cookie string.

use url::Url;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

pub type GetCookieResponse = String;

/// Get the current cookie string.
///
/// Mirrors the TypeScript client, which returns `ctx.cookie` — the cookie
/// jar contents for `https://chat.zalo.me`.
pub async fn get_cookie(client: &ApiClient) -> Result<GetCookieResponse> {
    let guard = client.ctx().http.lock().await;
    let http = guard
        .as_ref()
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("HTTP client not initialized")))?;
    let store = http
        .cookie_store
        .lock()
        .map_err(|_| ZaloError::Api(ZaloApiError::new("Cookie store lock poisoned")))?;
    let url = Url::parse("https://chat.zalo.me/")
        .map_err(|e| ZaloError::Api(ZaloApiError::new(format!("Invalid cookie URL: {e}"))))?;
    let cookies: Vec<String> = store
        .get_request_values(&url)
        .map(|(name, value)| format!("{name}={value}"))
        .collect();
    Ok(cookies.join("; "))
}
