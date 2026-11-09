//! Get auto reply list.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};

use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::auto_reply::AutoReplyItem;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAutoReplyListResponse {
    #[serde(default)]
    pub item: Vec<AutoReplyItem>,
    #[serde(default)]
    pub version: i64,
}

/// Get auto reply list.
pub async fn get_auto_reply_list(client: &ApiClient) -> Result<GetAutoReplyListResponse> {
    let url = client.service_url("auto_reply", "/api/autoreply/list")?;
    let params = serde_json::json!({
        "version": 0,
        "cliLang": client.ctx().language,
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
