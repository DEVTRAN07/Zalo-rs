//! Get friend onlines.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFriendOnlinesStatus {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetFriendOnlinesResponse {
    #[serde(default)]
    pub predefine: Vec<String>,
    #[serde(rename = "ownerStatus")]
    #[serde(default)]
    pub owner_status: String,
    #[serde(default)]
    pub onlines: Vec<GetFriendOnlinesStatus>,
}

/// Get friend onlines.
pub async fn get_friend_onlines(client: &ApiClient) -> Result<GetFriendOnlinesResponse> {
    let url = client.service_url("profile", "/api/social/friend/onlines")?;
    let params = serde_json::json!({
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
    let mut result: GetFriendOnlinesResponse = client.resolve(res).await?;
    for online in &mut result.onlines {
        if let Ok(parsed) = serde_json::from_str::<Value>(&online.status) {
            if let Some(status) = parsed.get("status").and_then(|v| v.as_str()) {
                online.status = status.to_string();
            }
        }
    }
    Ok(result)
}

