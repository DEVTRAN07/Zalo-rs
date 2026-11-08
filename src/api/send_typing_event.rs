use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::{DestType, ThreadType};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendTypingEventResponse {
    #[serde(default)]
    pub status: Value,
}

/// Send typing event.
pub async fn send_typing_event(
    client: &ApiClient,
    thread_id: &str,
    thread_type: ThreadType,
    dest_type: Option<DestType>,
) -> Result<SendTypingEventResponse> {
    if thread_id.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing threadId")));
    }

    let is_group = matches!(thread_type, ThreadType::Group);
    let (base_svc, path) = if is_group {
        ("group", "/api/group/typing")
    } else {
        ("chat", "/api/message/typing")
    };
    let base_url = client.service_url(base_svc, path)?;
    let url = client.make_url(&base_url, &[], true)?;

    let mut params = serde_json::json!({
        "imei": client.ctx().imei,
    });
    if is_group {
        params["grid"] = Value::String(thread_id.to_string());
    } else {
        params["toid"] = Value::String(thread_id.to_string());
        let dt = dest_type.unwrap_or(DestType::User) as i32;
        params["destType"] = Value::Number(dt.into());
    }

    client.post_encrypted(&url, &params).await
}
