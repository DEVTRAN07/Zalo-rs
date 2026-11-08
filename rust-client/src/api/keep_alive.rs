//! Keep-alive endpoint.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeepAliveResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Keep the session alive.
///
/// Uses the chat keepalive endpoint. The exact payload is minimal; failures
/// are non-fatal in typical usage.
pub async fn keep_alive(client: &ApiClient) -> Result<KeepAliveResponse> {
    // Zalo's keepAlive varies; use chat service as a common target.
    let url = client
        .service_url("chat", "/keepalive")
        .unwrap_or_else(|_| "https://wpa.chat.zalo.me/api/chat/keepalive".to_string());
    let url = client.make_url(&url, &[], true)?;

    let params = serde_json::json!({
        "imei": client.ctx().imei,
    });
    if let Some(encrypted) = client.encode_aes(&params.to_string()) {
        let res = client.request_post(&url, &[("params", encrypted)]).await?;
        // Try to resolve; if server returns unexpected format, return default.
        match client.resolve::<KeepAliveResponse>(res).await {
            Ok(v) => Ok(v),
            Err(_) => Ok(KeepAliveResponse::default()),
        }
    } else {
        Ok(KeepAliveResponse::default())
    }
}
