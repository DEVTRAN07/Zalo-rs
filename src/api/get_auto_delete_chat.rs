//! Get auto delete chat.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoDeleteConvers {
    #[serde(rename = "destId")]
    pub dest_id: String,
    #[serde(rename = "isGroup")]
    pub is_group: bool,
    pub ttl: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAutoDeleteChatResponse {
    #[serde(default)]
    pub convers: Vec<AutoDeleteConvers>,
}

/// Get auto delete chat.
pub async fn get_auto_delete_chat(client: &ApiClient) -> Result<GetAutoDeleteChatResponse> {
    let url = client.service_url("conversation", "/api/conv/autodelete/getConvers")?;
    let params = serde_json::json!({});

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
