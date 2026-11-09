//! Get archived chat list.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetArchivedChatListResponse {
    #[serde(default)]
    pub items: Vec<Value>,
    #[serde(default)]
    pub version: i64,
}

/// Get archived chat list.
pub async fn get_archived_chat_list(client: &ApiClient) -> Result<GetArchivedChatListResponse> {
    let url = client.service_url("label", "/api/archivedchat/list")?;
    let params = serde_json::json!({
        "version": 1,
        "imei": client.ctx().imei,
    });

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let full_url = if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(&encrypted))
    } else {
        format!("{}?params={}", url, urlencoding::encode(&encrypted))
    };
    let res = client.request_get(&full_url).await?;
    client.resolve(res).await
}
