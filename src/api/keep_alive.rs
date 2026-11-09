//! Keep-alive endpoint.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeepAliveResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Keep the session alive.
///
/// Uses the chat keepalive endpoint with a GET request and the encrypted
/// params placed in the URL query string (mirrors the TypeScript client).
/// The response is resolved as unencrypted, matching the TS client.
pub async fn keep_alive(client: &ApiClient) -> Result<KeepAliveResponse> {
    let url = client
        .service_url("chat", "/keepalive")
        .unwrap_or_else(|_| "https://wpa.chat.zalo.me/api/chat/keepalive".to_string());

    let params = serde_json::json!({
        "imei": client.ctx().imei,
    });

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve_plain(res).await
}
