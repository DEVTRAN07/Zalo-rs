//! Remove unread mark.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::{FactoryUtils, ZaloEnvelope};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveUnreadMarkResponse {
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub status: i64,
}

/// Remove unread mark from conversation.
pub async fn remove_unread_mark(
    ctx: &ContextSession,
    service_map: &Value,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<RemoveUnreadMarkResponse> {
    let base_url = service_url(service_map, "conversation", "/api/conv/removeUnreadMark")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let timestamp = chrono::Utc::now().timestamp_millis();
    let is_group = matches!(thread_type, ThreadType::Group);
    let param = if is_group {
        serde_json::json!({
            "convsGroup": [thread_id],
            "convsUser": [],
            "convsGroupData": [{ "id": thread_id, "ts": timestamp }],
            "convsUserData": [],
        })
    } else {
        serde_json::json!({
            "convsUser": [thread_id],
            "convsGroup": [],
            "convsUserData": [{ "id": thread_id, "ts": timestamp }],
            "convsGroupData": [],
        })
    };

    let request_params = serde_json::json!({ "param": param.to_string() });
    let encrypted = utils
        .encode_aes(&request_params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
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
