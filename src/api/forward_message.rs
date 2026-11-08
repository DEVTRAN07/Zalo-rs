use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForwardMessagePayloadReference {
    pub id: String,
    pub ts: i64,
    #[serde(rename = "logSrcType")]
    pub log_src_type: i64,
    #[serde(rename = "fwLvl")]
    pub fw_lvl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForwardMessagePayload {
    pub message: String,
    #[serde(default)]
    pub ttl: Option<i64>,
    #[serde(default)]
    pub reference: Option<ForwardMessagePayloadReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForwardMessageSuccess {
    #[serde(rename = "clientId", default)]
    pub client_id: String,
    #[serde(rename = "msgId", default)]
    pub msg_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForwardMessageFail {
    #[serde(rename = "clientId", default)]
    pub client_id: String,
    #[serde(default)]
    pub error_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForwardMessageResponse {
    #[serde(default)]
    pub success: Vec<ForwardMessageSuccess>,
    #[serde(default)]
    pub fail: Vec<ForwardMessageFail>,
}

/// Forward message to multiple threads.
pub async fn forward_message(
    client: &ApiClient,
    payload: ForwardMessagePayload,
    thread_ids: &[String],
    thread_type: ThreadType,
) -> Result<ForwardMessageResponse> {
    if payload.message.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing message content")));
    }
    if thread_ids.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing thread IDs")));
    }

    let is_group = matches!(thread_type, ThreadType::Group);
    let path = if is_group {
        "/api/group/mforward"
    } else {
        "/api/message/mforward"
    };
    let url = client.service_url("file", path)?;
    let url = client.make_url(&url, &[], true)?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let client_id = timestamp.to_string();

    let reference_json = if let Some(ref reference) = payload.reference {
        let ref_data = serde_json::json!({
            "id": reference.id,
            "ts": reference.ts,
            "logSrcType": reference.log_src_type,
            "fwLvl": reference.fw_lvl,
        });
        Some(serde_json::json!({
            "type": 3,
            "data": ref_data.to_string(),
        })
        .to_string())
    } else {
        None
    };

    let mut msg_info = serde_json::json!({
        "message": payload.message,
    });
    if let Some(r) = reference_json {
        msg_info["reference"] = Value::String(r);
    }

    let decor_log = if let Some(ref reference) = payload.reference {
        serde_json::json!({
            "fw": {
                "pmsg": {
                    "st": 1,
                    "ts": reference.ts,
                    "id": reference.id,
                },
                "rmsg": {
                    "st": 1,
                    "ts": reference.ts,
                    "id": reference.id,
                },
                "fwLvl": reference.fw_lvl,
            }
        })
    } else {
        Value::Null
    };

    let ttl = payload.ttl.unwrap_or(0);
    let params = if !is_group {
        serde_json::json!({
            "toIds": thread_ids.iter().map(|tid| serde_json::json!({
                "clientId": client_id,
                "toUid": tid,
                "ttl": ttl,
            })).collect::<Vec<_>>(),
            "imei": client.ctx().imei,
            "ttl": ttl,
            "msgType": "1",
            "totalIds": thread_ids.len(),
            "msgInfo": msg_info.to_string(),
            "decorLog": decor_log.to_string(),
        })
    } else {
        serde_json::json!({
            "grids": thread_ids.iter().map(|gid| serde_json::json!({
                "clientId": client_id,
                "grid": gid,
                "ttl": ttl,
            })).collect::<Vec<_>>(),
            "ttl": ttl,
            "msgType": "1",
            "totalIds": thread_ids.len(),
            "msgInfo": msg_info.to_string(),
            "decorLog": decor_log.to_string(),
        })
    };

    client.post_encrypted(&url, &params).await
}
