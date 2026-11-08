//! HTTP engine: cookie-aware `reqwest` wrapper with retry and timeout.

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONTENT_TYPE, ORIGIN, REFERER, USER_AGENT};
use reqwest::Method;
use reqwest_cookie_store::CookieStoreMutex;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

use crate::context::{ContextBase, HttpState, RetryOptions};
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::{zlog_warn};

const ZALO_ORIGIN: &str = "https://chat.zalo.me";
#[allow(dead_code)]
const LOGIN_REFERER: &str = "https://id.zalo.me/";

const RETRYABLE_STATUS: &[u16] = &[408, 425, 429, 500, 502, 503, 504];

fn is_retryable_status(status: u16) -> bool {
    RETRYABLE_STATUS.contains(&status)
}

fn is_retryable_method(method: &Method, policy: &RetryOptions) -> bool {
    policy
        .methods
        .iter()
        .any(|m| m.eq_ignore_ascii_case(method.as_str()))
}

/// Builds a Zalo API URL, appending query params and (by default) the API
/// version/type markers.
pub fn make_url(
    ctx: &ContextBase,
    base_url: &str,
    params: &[(&str, String)],
    api_version: bool,
) -> Result<String> {
    let mut url = Url::parse(base_url)?;
    {
        let mut pairs = url.query_pairs_mut();
        for (k, v) in params {
            pairs.append_pair(k, v);
        }
        if api_version {
            // Only add if not present
            drop(pairs);
            let has_ver = url.query_pairs().any(|(k, _)| k == "zpw_ver");
            let has_type = url.query_pairs().any(|(k, _)| k == "zpw_type");
            let mut pairs = url.query_pairs_mut();
            if !has_ver {
                pairs.append_pair("zpw_ver", &ctx.api_version.to_string());
            }
            if !has_type {
                pairs.append_pair("zpw_type", &ctx.api_type.to_string());
            }
        }
    }
    Ok(url.to_string())
}

/// Build default browser-like headers.
pub fn default_headers(user_agent: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert(ORIGIN, HeaderValue::from_static(ZALO_ORIGIN));
    headers.insert(REFERER, HeaderValue::from_static("https://chat.zalo.me/"));
    if let Ok(v) = HeaderValue::from_str(user_agent) {
        headers.insert(USER_AGENT, v);
    }
    headers
}

/// Initialize an HTTP client with the given cookie jar.
pub async fn init_http(ctx: &ContextBase, cookie_store: Arc<CookieStoreMutex>) -> Result<()> {
    let client = reqwest::Client::builder()
        .cookie_provider(cookie_store.clone())
        .gzip(true)
        .brotli(true)
        .deflate(true)
        .build()?;
    let mut guard = ctx.http.lock().await;
    *guard = Some(HttpState { client, cookie_store });
    Ok(())
}

/// Get the HTTP client, or error if not initialized.
pub async fn get_http(ctx: &ContextBase) -> Result<HttpState> {
    let guard = ctx.http.lock().await;
    guard
        .clone()
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("HTTP client not initialized (no cookies/session)")))
}

fn retry_delay_ms(policy: &RetryOptions, attempt: u32, retry_after: Option<&str>) -> u64 {
    if let Some(ra) = retry_after {
        if let Ok(secs) = ra.trim().parse::<f64>() {
            if secs >= 0.0 {
                return ((secs * 1000.0) as u64).min(policy.max_delay_ms);
            }
        }
    }
    let exponential = (policy.delay_ms.saturating_mul(2u64.saturating_pow(attempt))).min(policy.max_delay_ms);
    let jitter = rand::random::<u64>() % (exponential / 2 + 1);
    exponential / 2 + jitter
}

/// Perform an HTTP request with retry, timeout, and cookie handling.
///
/// `raw=true` skips default header injection (cookies still handled by the jar).
pub async fn request(
    ctx: &ContextBase,
    method: Method,
    url: &str,
    body: Option<String>,
    extra_headers: Option<HeaderMap>,
    raw: bool,
) -> Result<reqwest::Response> {
    let policy = &ctx.options.retry;
    let retryable = is_retryable_method(&method, policy);
    let http = get_http(ctx).await?;
    let user_agent = ctx.user_agent.clone().unwrap_or_default();

    for attempt in 0..=policy.limit {
        let mut headers = if raw {
            HeaderMap::new()
        } else {
            default_headers(&user_agent)
        };
        if let Some(extra) = &extra_headers {
            for (k, v) in extra.iter() {
                headers.insert(k.clone(), v.clone());
            }
        }

        let mut req = http.client.request(method.clone(), url).headers(headers);
        if let Some(b) = &body {
            req = req.body(b.clone());
        }
        if policy.timeout_ms > 0 {
            req = req.timeout(Duration::from_millis(policy.timeout_ms));
        }

        match req.send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let exhausted = attempt >= policy.limit;
                if !retryable || exhausted || resp.status().is_success() || !is_retryable_status(status) {
                    return Ok(resp);
                }
                let retry_after = resp
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string());
                let wait = retry_delay_ms(policy, attempt, retry_after.as_deref());
                zlog_warn!(
                    "Request to {} failed with status {}, retrying in {}ms ({}/{})",
                    url,
                    status,
                    wait,
                    attempt + 1,
                    policy.limit
                );
                tokio::time::sleep(Duration::from_millis(wait)).await;
            }
            Err(e) => {
                let exhausted = attempt >= policy.limit;
                if !retryable || exhausted {
                    return Err(e.into());
                }
                let wait = retry_delay_ms(policy, attempt, None);
                zlog_warn!(
                    "Request to {} failed ({}), retrying in {}ms ({}/{})",
                    url,
                    e,
                    wait,
                    attempt + 1,
                    policy.limit
                );
                tokio::time::sleep(Duration::from_millis(wait)).await;
            }
        }
    }

    Err(ZaloError::Api(ZaloApiError::new("request: unreachable retry state")))
}

/// Convenience: POST with form-encoded body.
pub async fn post_form(ctx: &ContextBase, url: &str, form: &[(&str, String)]) -> Result<reqwest::Response> {
    let body = form
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    request(ctx, Method::POST, url, Some(body), None, false).await
}

/// Convenience: GET.
pub async fn get(ctx: &ContextBase, url: &str) -> Result<reqwest::Response> {
    request(ctx, Method::GET, url, None, None, false).await
}
