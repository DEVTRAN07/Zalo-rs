//! Send message endpoint: text messages with optional mentions, quotes,
//! attachments, text styles, urgency and TTL (mirrors the TypeScript source).

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::api::upload_attachment::AttachmentSource;
use crate::models::r#enum::ThreadType;

/// Message urgency, mirrored into `metaData.urgency` (matches the TS `Urgency` enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Urgency {
    Default = 0,
    Important = 1,
    Urgent = 2,
}

/// Typed response for send_message and send_message_options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub msg_id: i64,
}

/// Options for [`send_message_options`], mirroring the TypeScript `MessageContent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageOptions {
    pub message: String,
    pub thread_id: String,
    pub thread_type: ThreadType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_info: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_msg_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_cli_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_ttl: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_properties: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<Urgency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<i32>,
}

fn insert_opt(map: &mut Map<String, Value>, key: &str, value: Option<Value>) {
    if let Some(value) = value {
        map.insert(key.to_string(), value);
    }
}

/// Extract msg_id from API response (handles both msgId and msg_id).
fn extract_msg_id(response: Value) -> Result<i64> {
    response.get("msgId")
        .or_else(|| response.get("msg_id"))
        .and_then(|v| v.as_i64())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Missing msg_id in response")))
}

/// Send a plain text message.
///
/// `thread_type`: 0 = user, 1 = group.
///
/// This preserves the original minimal behaviour when no extra features are used.
pub async fn send_message(
    client: &ApiClient,
    message: &str,
    thread_id: &str,
    thread_type: u8,
) -> Result<SendMessageResponse> {
    if message.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing message content")));
    }
    if thread_id.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing thread_id")));
    }

    let is_group = thread_type == 1;
    let (service, path) = if is_group {
        ("group", "/api/group/sendmsg")
    } else {
        ("chat", "/api/message/sms")
    };

    let url = client.service_url(service, path)?;
    let url = client.make_url(&url, &[], true)?;

    let client_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let params = if is_group {
        serde_json::json!({
            "grid": thread_id,
            "message": message,
            "clientId": client_id,
            "visibility": 0,
            "imei": client.ctx().imei,
        })
    } else {
        serde_json::json!({
            "toId": thread_id,
            "message": message,
            "clientId": client_id,
            "imei": client.ctx().imei,
            "mentionInfo": "",
        })
    };

    let response = client.post_encrypted(&url, &params).await?;
    let msg_id = extract_msg_id(response)?;
    Ok(SendMessageResponse { msg_id })
}

/// Send a message with optional mentions, quotes, attachments, text styles,
/// urgency and TTL.
///
/// The URL path is chosen based on the features present:
/// - quote fields  → `/api/{message,group}/quote`
/// - mention (group only) → `/api/group/mention`
/// - otherwise     → `/api/message/sms` (user) or `/api/group/sendmsg` (group)
///
/// When only a plain text message is given the behaviour matches
/// [`send_message`].
pub async fn send_message_options(client: &ApiClient, options: SendMessageOptions) -> Result<SendMessageResponse> {
    let SendMessageOptions {
        message,
        thread_id,
        thread_type,
        mention_info,
        quote_msg_id,
        quote_owner_id,
        quote_cli_id,
        quote_type,
        quote_ts,
        quote_data,
        quote_ttl,
        attachments,
        text_properties,
        urgency,
        meta_data,
        ttl,
        visibility,
    } = options;

    if thread_id.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing thread_id")));
    }
    let has_attachments = attachments.as_ref().map_or(false, |src| !src.is_empty());
    if message.is_empty() && !has_attachments {
        return Err(ZaloError::Api(ZaloApiError::new("Missing message content")));
    }

    let is_group = thread_type == ThreadType::Group;
    let service = if is_group { "group" } else { "chat" };
    let root = if is_group { "/api/group" } else { "/api/message" };

    let quote_present = quote_msg_id.is_some()
        || quote_owner_id.is_some()
        || quote_cli_id.is_some()
        || quote_type.is_some()
        || quote_ts.is_some()
        || quote_data.is_some()
        || quote_ttl.is_some();
    let mention_present = mention_info.is_some();

    let path = if quote_present {
        format!("{}/quote", root)
    } else if mention_present && is_group {
        format!("{}/mention", root)
    } else if is_group {
        "/api/group/ssendmsg".to_string()
    } else {
        "/api/message/sms".to_string()
    };

    let url = client.service_url(service, &path)?;
    let url = client.make_url(&url, &[("nretry", "0".to_string())], true)?;

    let client_id = chrono::Utc::now().timestamp_millis();

    let message_empty = message.is_empty();
    let mut params = Map::new();
    params.insert("message".to_string(), Value::String(message));
    params.insert("clientId".to_string(), Value::from(client_id));
    if !is_group || !quote_present {
        params.insert("imei".to_string(), Value::String(client.ctx().imei.clone()));
    }
    if is_group {
        params.insert("grid".to_string(), Value::String(thread_id.clone()));
        params.insert("visibility".to_string(), Value::from(visibility.unwrap_or(0)));
    } else {
        params.insert("toid".to_string(), Value::String(thread_id.clone()));
    }
    if is_group {
        insert_opt(&mut params, "mentionInfo", mention_info);
    }
    params.insert("ttl".to_string(), Value::from(ttl.unwrap_or(0)));
    insert_opt(&mut params, "textProperties", text_properties);
    let meta = meta_data.or_else(|| {
        urgency.map(|u| serde_json::json!({ "urgency": u as i64 }))
    });
    insert_opt(&mut params, "metaData", meta);

    if quote_present {
        insert_opt(&mut params, "qmsgOwner", quote_owner_id.map(Value::String));
        insert_opt(&mut params, "qmsgId", quote_msg_id.map(Value::String));
        insert_opt(&mut params, "qmsgCliId", quote_cli_id.map(Value::String));
        insert_opt(&mut params, "qmsgType", quote_type.map(Value::String));
        insert_opt(&mut params, "qmsgTs", quote_ts.map(Value::from));
        insert_opt(&mut params, "qmsg", quote_data.map(Value::String));
        insert_opt(&mut params, "qmsgTTL", quote_ttl.map(Value::from));
        if is_group {
            params.insert("visibility".to_string(), Value::from(0));
        }
    }

    let params = Value::Object(params);

    if has_attachments {
        let sources = attachments.expect("presence checked above");
        let uploaded =
            crate::api::upload_attachment::upload_attachment(client, sources, &thread_id, thread_type)
                .await?;
        if message_empty {
            let msg_id = extract_msg_id(serde_json::to_value(&uploaded).unwrap_or(Value::Null))?;
            return Ok(SendMessageResponse { msg_id });
        }
    }

    let response = client.post_encrypted(&url, &params).await?;
    let msg_id = extract_msg_id(response)?;
    Ok(SendMessageResponse { msg_id })
}
