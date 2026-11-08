use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUnreadMarkResponse {
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub status: i64,
}

#[derive(Debug, Deserialize)]
struct ZaloEnvelope<T> {
    #[serde(rename = "error_code")]
    error_code: i64,
    #[serde(rename = "error_message")]
    error_message: Option<String>,
    data: Option<T>,
}

/// Get unread mark.
pub async fn get_unread_mark(client: &ApiClient) -> Result<GetUnreadMarkResponse> {
    let base_url = client.service_url("conversation", "/api/conv/getUnreadMark")?;
    let params = serde_json::json!({});
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = url_with_params(&base_url, &encrypted);
    let res = client.request_get(&url).await?;
    if !res.status().is_success() {
        return Err(ZaloError::Api(ZaloApiError::new(format!(
            "Request failed with status code {}",
            res.status()
        ))));
    }
    let envelope: ZaloEnvelope<Value> = res
        .json()
        .await
        .map_err(|e| ZaloError::Api(ZaloApiError::new(e.to_string())))?;
    if envelope.error_code != 0 {
        return Err(ZaloError::Api(ZaloApiError {
            code: Some(envelope.error_code),
            message: envelope.error_message.unwrap_or_else(|| "Unknown error".into()),
        }));
    }
    let mut data = envelope.data.unwrap_or(Value::Null);
    if let Some(s) = data.get("data").and_then(|v| v.as_str()) {
        if let Ok(parsed) = serde_json::from_str::<Value>(s) {
            if let Some(obj) = data.as_object_mut() {
                obj.insert("data".to_string(), parsed);
            }
        }
    }
    serde_json::from_value(data).map_err(|e| ZaloError::Api(ZaloApiError::new(e.to_string())))
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
