use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveQuickMessageResponse {
    #[serde(default)]
    pub item: Option<i64>,
    #[serde(default)]
    pub version: i64,
}

/// Remove quick message.
pub async fn remove_quick_message(
    client: &ApiClient,
    item_ids: &[i64],
) -> Result<RemoveQuickMessageResponse> {
    let base_url = client.service_url("quick_message", "/api/quickmessage/delete")?;
    let params = serde_json::json!({
        "itemIds": serde_json::to_string(item_ids).unwrap_or_else(|_| "[]".to_string()),
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = if base_url.contains('?') { format!("{}&params={}", &base_url, urlencoding::encode(&encrypted)) } else { format!("{}?params={}", &base_url, urlencoding::encode(&encrypted)) };
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
