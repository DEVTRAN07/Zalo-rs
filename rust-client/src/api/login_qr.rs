//! QR-code login flow (pre-authentication, uses `ContextBase`).
//!
//! This is a best-effort port of the TypeScript `loginQR` flow. It provides
//! the low-level HTTP steps; interactive callback/UI handling is left to the
//! caller. The flow:
//! 1. `load_login_page` to discover the login JS version
//! 2. `get_login_info` / `verify_client`
//! 3. `generate_qr` to get QR image + code
//! 4. `waiting_scan` (polls) then `waiting_confirm` (polls)
//! 5. `check_session` + `get_user_info_qr` to finalize
//!
//! Note: cookie-jar mutation across requests relies on the shared `reqwest`
//! client in `ContextBase.http` having cookie store enabled. The TS version
//! used `tough-cookie`; in Rust, `reqwest::Client` with `cookie_store(true)`
//! provides equivalent behavior when built accordingly.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::ContextBase;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::http;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QrGenerateData {
    pub code: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub options: Value,
    #[serde(default)]
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QrGenerateResponse {
    #[serde(default)]
    pub data: Option<QrGenerateData>,
    #[serde(default)]
    pub error_code: i64,
    #[serde(default)]
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QrScanData {
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QrScanResponse {
    #[serde(default)]
    pub data: Option<QrScanData>,
    #[serde(default)]
    pub error_code: i64,
    #[serde(default)]
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QrConfirmResponse {
    #[serde(default)]
    pub error_code: i64,
    #[serde(default)]
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QrUserInfoData {
    #[serde(default)]
    pub logged: bool,
    #[serde(default)]
    pub session_chat_valid: bool,
    #[serde(default)]
    pub info: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QrUserInfoResponse {
    #[serde(default)]
    pub data: Option<QrUserInfoData>,
    #[serde(default)]
    pub error_code: i64,
    #[serde(default)]
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginQrResult {
    pub user_info: Value,
    pub login_version: String,
}

/// Discover the login page JS version.
pub async fn load_login_page(ctx: &ContextBase) -> Result<String> {
    let url = "https://id.zalo.me/account?continue=https%3A%2F%2Fchat.zalo.me%2F";
    let res = http::get(ctx, url).await?;
    let html = res.text().await?;
    // Match https://stc-zlogin.zdn.vn/main-<version>.js
    let marker = "https://stc-zlogin.zdn.vn/main-";
    if let Some(start) = html.find(marker) {
        let rest = &html[start + marker.len()..];
        if let Some(end) = rest.find(".js") {
            return Ok(rest[..end].to_string());
        }
    }
    Err(ZaloError::Api(ZaloApiError::new(
        "Cannot get API login version",
    )))
}

async fn post_id_form(ctx: &ContextBase, url: &str, params: &[(&str, &str)]) -> Result<Value> {
    let form: Vec<(&str, String)> = params.iter().map(|(k, v)| (*k, v.to_string())).collect();
    let res = http::post_form(ctx, url, &form).await?;
    let data: Value = res.json().await?;
    Ok(data)
}

pub async fn get_login_info(ctx: &ContextBase, version: &str) -> Result<Value> {
    post_id_form(
        ctx,
        "https://id.zalo.me/account/logininfo",
        &[("continue", "https://zalo.me/pc"), ("v", version)],
    )
    .await
}

pub async fn verify_client(ctx: &ContextBase, version: &str) -> Result<Value> {
    post_id_form(
        ctx,
        "https://id.zalo.me/account/verify-client",
        &[
            ("type", "device"),
            ("continue", "https://zalo.me/pc"),
            ("v", version),
        ],
    )
    .await
}

pub async fn generate_qr(ctx: &ContextBase, version: &str) -> Result<QrGenerateResponse> {
    let data = post_id_form(
        ctx,
        "https://id.zalo.me/account/authen/qr/generate",
        &[("continue", "https://zalo.me/pc"), ("v", version)],
    )
    .await?;
    Ok(serde_json::from_value(data).unwrap_or_default())
}

pub async fn waiting_scan(ctx: &ContextBase, version: &str, code: &str) -> Result<QrScanResponse> {
    loop {
        let data = post_id_form(
            ctx,
            "https://id.zalo.me/account/authen/qr/waiting-scan",
            &[
                ("code", code),
                ("continue", "https://chat.zalo.me/"),
                ("v", version),
            ],
        )
        .await?;
        let resp: QrScanResponse = serde_json::from_value(data).unwrap_or_default();
        if resp.error_code == 8 {
            continue;
        }
        return Ok(resp);
    }
}

pub async fn waiting_confirm(
    ctx: &ContextBase,
    version: &str,
    code: &str,
) -> Result<QrConfirmResponse> {
    loop {
        let data = post_id_form(
            ctx,
            "https://id.zalo.me/account/authen/qr/waiting-confirm",
            &[
                ("code", code),
                ("gToken", ""),
                ("gAction", "CONFIRM_QR"),
                ("continue", "https://chat.zalo.me/"),
                ("v", version),
            ],
        )
        .await?;
        let resp: QrConfirmResponse = serde_json::from_value(data).unwrap_or_default();
        if resp.error_code == 8 {
            continue;
        }
        return Ok(resp);
    }
}

pub async fn check_session(ctx: &ContextBase) -> Result<reqwest::Response> {
    http::get(
        ctx,
        "https://id.zalo.me/account/checksession?continue=https%3A%2F%2Fchat.zalo.me%2Findex.html",
    )
    .await
}

pub async fn get_user_info_qr(ctx: &ContextBase) -> Result<QrUserInfoResponse> {
    let res = http::get(ctx, "https://jr.chat.zalo.me/jr/userinfo").await?;
    let data: Value = res.json().await?;
    Ok(serde_json::from_value(data).unwrap_or_default())
}

/// Run the QR login flow (non-interactive skeleton).
///
/// Returns the generated QR data and login version. The caller is responsible
/// for displaying the QR, then calling `waiting_scan`, `waiting_confirm`,
/// `check_session`, and `get_user_info_qr` to complete the flow.
pub async fn login_qr_start(ctx: &ContextBase) -> Result<(String, QrGenerateResponse)> {
    let version = load_login_page(ctx).await?;
    let _ = get_login_info(ctx, &version).await;
    let _ = verify_client(ctx, &version).await;
    let qr = generate_qr(ctx, &version).await?;
    if qr.data.is_none() {
        return Err(ZaloError::Api(ZaloApiError::new(format!(
            "Unable to generate QRCode: {}",
            qr.error_message
        ))));
    }
    Ok((version, qr))
}
