use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateQuickMessagePayload {
    pub keyword: String,
    pub title: String,
    /// Reserved for media attachment (TS uploads via `uploadProductPhoto`).
    /// In Rust, pass pre-uploaded media info via `media_items` if needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_items: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateQuickMessageResponse {
    #[serde(default)]
    pub item: Value,
    #[serde(default)]
    pub version: i64,
}

/// Update a quick message by item id.
pub async fn update_quick_message(
    client: &ApiClient,
    payload: UpdateQuickMessagePayload,
    item_id: i64,
) -> Result<UpdateQuickMessageResponse> {
    let msg_type = if payload.media_items.is_some() { 1 } else { 0 };
    let mut params = serde_json::json!({
        "itemId": item_id,
        "keyword": payload.keyword,
        "message": {
            "title": payload.title,
            "params": "",
        },
        "type": msg_type,
    });
    if let Some(items) = payload.media_items {
        params["media"] = serde_json::json!({ "items": items });
    }

    let base_url = client.service_url("quick_message", "/api/quickmessage/update")?;
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
