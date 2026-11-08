//! Send message delivered event.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::ThreadType;

pub type SendDeliveredEventResponse = Value;

const MAX_MESSAGES_PER_SEND: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendDeliveredEventMessageParams {
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "cliMsgId")]
    pub cli_msg_id: String,
    #[serde(rename = "uidFrom")]
    pub uid_from: String,
    #[serde(rename = "idTo")]
    pub id_to: String,
    #[serde(rename = "msgType")]
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
    ctx: &ContextSession,
    service_map: &Value,
    is_seen: bool,
    messages: &[SendDeliveredEventMessageParams],
    thread_type: ThreadType,
) -> Result<SendDeliveredEventResponse> {
    if messages.is_empty() || messages.len() > MAX_MESSAGES_PER_SEND {
        return Err(ZaloError::Api(ZaloApiError::new(
            "messages must contain between 1 and 50 messages.",
        )));
    }

    let id_to = &messages[0].id_to;
    if matches!(thread_type, ThreadType::Group) && !messages.iter().all(|m| &m.id_to == id_to) {
        return Err(ZaloError::Api(ZaloApiError::new(
            "All messages must have the same idTo for Group thread",
        )));
    }

    let data: Vec<Value> = messages
        .iter()
        .map(|msg| {
            let ts = msg
                .ts
                .as_i64()
                .or_else(|| msg.ts.as_str().and_then(|s| s.parse::<i64>().ok()))
                .unwrap_or(0);
            serde_json::json!({
                "cmi": msg.cli_msg_id,
                "gmi": msg.msg_id,
                "si": msg.uid_from,
                "di": if msg.id_to == ctx.uid { "0" } else { &msg.id_to },
                "mt": msg.msg_type,
                "st": msg.st,
                "at": msg.at,
                "cmd": msg.cmd,
                "ts": ts,
            })
        })
        .collect();

    let mut msg_infos = serde_json::json!({
        "seen": if is_seen { 1 } else { 0 },
        "data": data,
    });
    if matches!(thread_type, ThreadType::Group) {
        msg_infos["grid"] = Value::String(id_to.clone());
    }

    let mut params = serde_json::json!({ "msgInfos": msg_infos.to_string() });
    let path = match thread_type {
        ThreadType::User => "/api/message/deliveredv2",
        ThreadType::Group => "/api/group/deliveredv2",
    };
    if matches!(thread_type, ThreadType::Group) {
        params["imei"] = Value::String(ctx.imei.clone());
    }

    let base_url = service_url(service_map, if matches!(thread_type, ThreadType::Group) { "group" } else { "chat" }, path)?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
