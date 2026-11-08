use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloError};
use crate::models::r#enum::ThreadType;

pub type SendDeliveredEventResponse = Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveredMessage {
    #[serde(rename = "msgId", default)]
    pub msg_id: String,
    #[serde(rename = "cliMsgId", default)]
    pub cli_msg_id: String,
    #[serde(default)]
    pub uid_from: String,
    #[serde(default)]
    pub id_to: String,
    #[serde(default)]
    pub msg_type: String,
    #[serde(default)]
    pub st: i64,
    #[serde(default)]
    pub at: i64,
    #[serde(default)]
    pub cmd: i64,
    #[serde(default)]
    pub ts: Value,
}

/// Send message delivered event.
pub async fn send_delivered_event(
    client: &ApiClient,
    is_seen: bool,
    messages: &[DeliveredMessage],
    thread_type: ThreadType,
) -> Result<SendDeliveredEventResponse> {
    let is_group = matches!(thread_type, ThreadType::Group);
    let service = if is_group && is_seen { "group" } else { "chat" };
    let path = if is_group && is_seen {
        "/api/group/seens"
    } else if is_group {
        "/api/group/delivereds"
    } else if is_seen {
        "/api/message/seens"
    } else {
        "/api/message/delivereds"
    };

    let mut msg_infos: std::collections::HashMap<String, Value> = std::collections::HashMap::new();
    if is_group && is_seen {
        for msg in messages {
            if msg.id_to.is_empty() { continue; }
            if !msg_infos.contains_key(&msg.id_to) {
                msg_infos.insert(
                    msg.id_to.clone(),
                    serde_json::json!({ "groupMsgs": [], "gId": msg.id_to, "seen": 0 }),
                );
            }
            let entry = msg_infos.get_mut(&msg.id_to).unwrap();
            let mut gmsg = serde_json::json!({
                "cmi": msg.cli_msg_id,
                "gmi": msg.msg_id,
                "si": msg.uid_from,
            });
            if let Some(ts) = msg.ts.as_i64() {
                gmsg["ts"] = Value::from(ts);
            } else if let Some(ts) = msg.ts.as_str() {
                gmsg["ts"] = Value::from(ts);
            }
            if let Some(arr) = entry.get_mut("groupMsgs").and_then(|v| v.as_array_mut()) {
                arr.push(gmsg);
            }
        }
    } else if is_group {
        for msg in messages {
            if msg.id_to.is_empty() { continue; }
            if !msg_infos.contains_key(&msg.id_to) {
                msg_infos.insert(msg.id_to.clone(), serde_json::json!({ "groupMsgs": [] }));
            }
            let entry = msg_infos.get_mut(&msg.id_to).unwrap();
            if let Some(arr) = entry.get_mut("groupMsgs").and_then(|v| v.as_array_mut()) {
                arr.push(serde_json::json!({
                    "cmi": msg.cli_msg_id,
                    "gmi": msg.msg_id,
                    "si": msg.uid_from,
                    "ts": msg.ts,
                }));
            }
        }
    } else {
        for msg in messages {
            if msg.uid_from.is_empty() { continue; }
            let st = if msg.msg_type == "webchat" { 2 } else { if msg.st == 0 { -1 } else { msg.st } };
            let at = if msg.msg_type == "webchat" { 0 } else { msg.at };
            msg_infos.insert(
                msg.cli_msg_id.clone(),
                serde_json::json!({
                    "cmi": msg.cli_msg_id,
                    "gmi": msg.msg_id,
                    "si": msg.uid_from,
                    "di": msg.id_to,
                    "mt": if msg.msg_type.is_empty() { 1 } else { msg.msg_type.parse::<i64>().unwrap_or(1) },
                    "st": st,
                    "at": at,
                    "cmd": msg.cmd,
                    "ts": if msg.ts.as_i64().is_some() { msg.ts.clone() } else { Value::from(msg.ts.as_str().unwrap_or("").parse::<i64>().unwrap_or(0)) },
                }),
            );
        }
    }

    if msg_infos.is_empty() {
        return Err(ZaloError::Api(crate::errors::ZaloApiError::new("messages are empty")));
    }

    let base_url = client.service_url(service, path)?;
    let url = client.make_url(&base_url, &[], true)?;

    let mut params = serde_json::json!({
        "msgInfos": serde_json::to_string(&msg_infos).unwrap_or_else(|_| "{}".to_string()),
    });
    if is_seen && !is_group {
        let grid = if let Some(m) = messages.get(0) { m.id_to.clone() } else { String::new() };
        params["grid"] = Value::String(grid);
        params["imei"] = Value::String(client.ctx().imei.clone());
    } else if !is_group {
        params["imei"] = Value::String(client.ctx().imei.clone());
    }
    if is_group && !is_seen {
        params["imei"] = Value::String(client.ctx().imei.clone());
    }
    if is_group && is_seen {
        params["imei"] = Value::String(client.ctx().imei.clone());
    }

    client.post_encrypted(&url, &params).await
}
