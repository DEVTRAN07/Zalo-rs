//! API factory helpers: response parsing and encryption utilities.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::context::{ContextBase, ContextSession};
use crate::crypto::{decode_aes, encode_aes};
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::http;

/// Standard Zalo API response envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZaloEnvelope<T = serde_json::Value> {
    #[serde(default)]
    pub error_code: i64,
    #[serde(default)]
    pub error_message: Option<String>,
    #[serde(default)]
    pub data: Option<T>,
}

/// Parsed result from a Zalo API response.
#[derive(Debug, Clone)]
pub struct ZaloResponse<T> {
    pub data: Option<T>,
    pub error: Option<ZaloResponseError>,
}

#[derive(Debug, Clone)]
pub struct ZaloResponseError {
    pub message: String,
    pub code: Option<i64>,
}

/// Reads a Zalo API response, optionally decrypting its AES payload.
pub async fn handle_zalo_response<T: DeserializeOwned>(
    ctx: &ContextSession,
    response: reqwest::Response,
    is_encrypted: bool,
) -> ZaloResponse<T> {
    if !response.status().is_success() {
        return ZaloResponse {
            data: None,
            error: Some(ZaloResponseError {
                message: format!("Request failed with status code {}", response.status()),
                code: None,
            }),
        };
    }

    let json: serde_json::Value = match response.json().await {
        Ok(v) => v,
        Err(e) => {
            return ZaloResponse {
                data: None,
                error: Some(ZaloResponseError {
                    message: format!("Failed to parse response JSON: {}", e),
                    code: None,
                }),
            }
        }
    };

    let error_code = json.get("error_code").and_then(|v| v.as_i64()).unwrap_or(0);
    if error_code != 0 {
        let msg = json
            .get("error_message")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error")
            .to_string();
        return ZaloResponse {
            data: None,
            error: Some(ZaloResponseError {
                message: msg,
                code: Some(error_code),
            }),
        };
    }

    let payload = if is_encrypted {
        let data_str = match json.get("data").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => {
                return ZaloResponse {
                    data: None,
                    error: Some(ZaloResponseError {
                        message: "Encrypted response missing data field".into(),
                        code: None,
                    }),
                }
            }
        };
        match decode_aes(&ctx.secret_key, data_str) {
            Some(decoded) => match serde_json::from_str::<serde_json::Value>(&decoded) {
                Ok(v) => v,
                Err(e) => {
                    return ZaloResponse {
                        data: None,
                        error: Some(ZaloResponseError {
                            message: format!("Failed to parse decrypted payload: {}", e),
                            code: None,
                        }),
                    }
                }
            },
            None => {
                return ZaloResponse {
                    data: None,
                    error: Some(ZaloResponseError {
                        message: "Failed to decrypt response data".into(),
                        code: None,
                    }),
                }
            }
        }
    } else {
        json
    };

    // The decrypted payload may itself have error_code/data
    let inner_code = payload
        .get("error_code")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    if inner_code != 0 {
        let msg = payload
            .get("error_message")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error")
            .to_string();
        return ZaloResponse {
            data: None,
            error: Some(ZaloResponseError {
                message: msg,
                code: Some(inner_code),
            }),
        };
    }

    let data_val = payload.get("data").cloned().unwrap_or(payload);
    match serde_json::from_value::<T>(data_val) {
        Ok(data) => ZaloResponse {
            data: Some(data),
            error: None,
        },
        Err(e) => ZaloResponse {
            data: None,
            error: Some(ZaloResponseError {
                message: format!("Failed to deserialize response data: {}", e),
                code: None,
            }),
        },
    }
}

/// Resolve a response into plain data, returning `ZaloApiError` on failure.
pub async fn resolve_response<T: DeserializeOwned>(
    ctx: &ContextSession,
    res: reqwest::Response,
    is_encrypted: bool,
) -> Result<T> {
    let result = handle_zalo_response::<T>(ctx, res, is_encrypted).await;
    if let Some(err) = result.error {
        return Err(ZaloError::Api(match err.code {
            Some(code) => ZaloApiError::with_code(err.message, code),
            None => ZaloApiError::new(err.message),
        }));
    }
    result
        .data
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Empty response data")))
}

/// Helpers passed to endpoint implementations.
#[deprecated(note = "Use `api::ApiClient` for idiomatic request handling.")]
pub struct FactoryUtils<'a> {
    pub ctx: &'a ContextSession,
}

impl<'a> FactoryUtils<'a> {
    pub fn new(ctx: &'a ContextSession) -> Self {
        Self { ctx }
    }

    /// Build a Zalo API URL with version/type params.
    pub fn make_url(
        &self,
        base_url: &str,
        params: &[(&str, String)],
        api_version: bool,
    ) -> Result<String> {
        // Convert session ctx to base-compatible view for make_url
        let base = ContextBase {
            api_type: self.ctx.api_type,
            api_version: self.ctx.api_version,
            imei: Some(self.ctx.imei.clone()),
            user_agent: Some(self.ctx.user_agent.clone()),
            language: Some(self.ctx.language.clone()),
            secret_key: Some(self.ctx.secret_key.clone()),
            uid: Some(self.ctx.uid.clone()),
            options: self.ctx.options.clone(),
            http: self.ctx.http.clone(),
            login_info: Some(self.ctx.login_info.clone()),
            settings: self.ctx.settings.clone(),
            extra_ver: self.ctx.extra_ver.clone(),
            upload_callbacks: self.ctx.upload_callbacks.clone(),
        };
        http::make_url(&base, base_url, params, api_version)
    }

    /// Encrypt data with the session secret key (base64). Returns base64 ciphertext.
    pub fn encode_aes(&self, data: &str) -> Option<String> {
        encode_aes(&self.ctx.secret_key, data)
    }

    /// POST form data to a URL.
    pub async fn request_post(
        &self,
        url: &str,
        form: &[(&str, String)],
    ) -> Result<reqwest::Response> {
        let base = self.ctx_to_base();
        http::post_form(&base, url, form).await
    }

    /// GET a URL.
    pub async fn request_get(&self, url: &str) -> Result<reqwest::Response> {
        let base = self.ctx_to_base();
        http::get(&base, url).await
    }

    /// Resolve an encrypted response into typed data.
    pub async fn resolve<T: DeserializeOwned>(&self, res: reqwest::Response) -> Result<T> {
        resolve_response::<T>(self.ctx, res, true).await
    }

    /// Resolve an unencrypted response into typed data.
    pub async fn resolve_plain<T: DeserializeOwned>(&self, res: reqwest::Response) -> Result<T> {
        resolve_response::<T>(self.ctx, res, false).await
    }

    fn ctx_to_base(&self) -> ContextBase {
        ContextBase {
            api_type: self.ctx.api_type,
            api_version: self.ctx.api_version,
            imei: Some(self.ctx.imei.clone()),
            user_agent: Some(self.ctx.user_agent.clone()),
            language: Some(self.ctx.language.clone()),
            secret_key: Some(self.ctx.secret_key.clone()),
            uid: Some(self.ctx.uid.clone()),
            options: self.ctx.options.clone(),
            http: self.ctx.http.clone(),
            login_info: Some(self.ctx.login_info.clone()),
            settings: self.ctx.settings.clone(),
            extra_ver: self.ctx.extra_ver.clone(),
            upload_callbacks: self.ctx.upload_callbacks.clone(),
        }
    }
}
