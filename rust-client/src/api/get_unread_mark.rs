//! Get unread mark.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::{FactoryUtils, ZaloEnvelope};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUnreadMarkResponse {
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub status: i64,
}

/// Get unread mark.
pub async fn get_unread_mark(
    ctx: &ContextSession,
    service_map: &Value,
) -> Result<GetUnreadMarkResponse> {
    let base_url = service_url(service_map, "conversation", "/api/conv/getUnreadMark")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({});
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_get(&url_with_params(&url, &encrypted)).await?;
    let envelope: ZaloEnvelope<Value> = {
        if !res.status().is_success() {
            return Err(ZaloError::Api(ZaloApiError::new(format!(
                "Request failed with status code {}",
                res.status()
            ))));
        }
        res.json().await.map_err(|e| ZaloError::Api(ZaloApiError::new(e.to_string())))?
    };
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
