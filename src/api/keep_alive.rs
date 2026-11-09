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
/// Uses the chat keepalive endpoint with a GET request and the encrypted
/// params placed in the URL query string (mirrors the TypeScript client).
pub async fn keep_alive(client: &ApiClient) -> Result<KeepAliveResponse> {
    let url = client
        .service_url("chat", "/keepalive")
        .unwrap_or_else(|_| "https://wpa.chat.zalo.me/api/chat/keepalive".to_string());

    let params = serde_json::json!({
        "imei": client.ctx().imei,
    });
    client.get_encrypted::<KeepAliveResponse>(&url, &params).await
}
