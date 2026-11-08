//! Add reaction to a message.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddReactionResponse {
    #[serde(rename = "msgIds", default)]
    pub msg_ids: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomReaction {
    #[serde(rename = "rType")]
    pub r_type: i64,
    pub source: i64,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddReactionMessageData {
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "cliMsgId")]
    pub cli_msg_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddReactionDestination {
    pub data: AddReactionMessageData,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "type")]
    pub thread_type: ThreadType,
}

fn reaction_spec(icon: &Value) -> Option<(i64, i64, String)> {
    if let Some(obj) = icon.as_object() {
        let r_type = obj.get("rType").and_then(|v| v.as_i64())?;
        let source = obj.get("source").and_then(|v| v.as_i64()).unwrap_or(6);
        let icon_str = obj.get("icon").and_then(|v| v.as_str())?.to_string();
        return Some((r_type, source, icon_str));
    }
    let name = icon.as_str()?;
    let (r_type, source) = match name {
        "HAHA" | "/-h" => (0, 6),
        "LIKE" | "/-strong" => (3, 6),
        "HEART" | "/-heart" => (5, 6),
        "WOW" | ":o" => (32, 6),
        "CRY" | ":-(" => (2, 6),
        "ANGRY" | ":-@" => (20, 6),
        "KISS" | ":-*" => (8, 6),
        "TEARS_OF_JOY" | ":')" => (7, 6),
        "SHIT" | "/-shit" => (66, 6),
        "ROSE" | "/-rose" => (120, 6),
        "BROKEN_HEART" | "/-break" => (65, 6),
        "DISLIKE" | "/-weak" => (4, 6),
        "LOVE" | ";xx" => (29, 6),
        "CONFUSED" | ";-/" => (51, 6),
        "WINK" | ";-)" => (45, 6),
        "FADE" | "/-fade" => (121, 6),
        "SUN" | "/-li" => (67, 6),
        "BIRTHDAY" | "/-bd" => (126, 6),
        "BOMB" | "/-bome" => (127, 6),
        "OK" | "/-ok" => (68, 6),
        "PEACE" | "/-v" => (69, 6),
        "THANKS" | "/-thanks" => (70, 6),
        "PUNCH" | "/-punch" => (71, 6),
        "SHARE" | "/-share" => (72, 6),
        "PRAY" | "_()_" => (73, 6),
        "NO" | "/-no" => (131, 6),
        "BAD" | "/-bad" => (132, 6),
        "LOVE_YOU" | "/-loveu" => (133, 6),
        "SAD" | "--b" => (1, 6),
        "VERY_SAD" | ":((" => (16, 6),
        "COOL" | "x-)" => (21, 6),
        "NERD" | "8-)" => (22, 6),
        "BIG_SMILE" | ";-d" => (23, 6),
        "SUNGLASSES" | "b-)" => (26, 6),
        "NEUTRAL" | ":--|" => (30, 6),
        "SAD_FACE" | "p-(" => (35, 6),
        "BYE" | ":-bye" => (36, 6),
        "SLEEPY" | "|-)" => (38, 6),
        "WIPE" | ":wipe" => (39, 6),
        "DIG" | ":-dig" => (42, 6),
        "ANGUISH" | "&-(" => (44, 6),
        "HANDCLAP" | ":handclap" => (46, 6),
        "ANGRY_FACE" | ">-|" => (47, 6),
        "F_CHAIR" | ":-f" => (48, 6),
        "L_CHAIR" | ":-l" => (49, 6),
        "R_CHAIR" | ":-r" => (50, 6),
        "SILENT" | ";-x" => (52, 6),
        "SURPRISE" | ":-o" => (53, 6),
        "EMBARRASSED" | ";-s" => (54, 6),
        "AFRAID" | ";-a" => (60, 6),
        "SAD2" | ":-<" => (61, 6),
        "BIG_LAUGH" | ":))" => (62, 6),
        "RICH" | "$-)" => (63, 6),
        "BEER" | "/-beer" => (99, 6),
        _ => (-1, 6),
    };
    Some((r_type, source, name.to_string()))
}

/// Add reaction to a message. `icon` can be a reaction name (e.g. "HEART")
/// or a custom object with `rType`, `source`, `icon`.
pub async fn add_reaction(
    client: &ApiClient,
    icon: Value,
    dest: AddReactionDestination,
) -> Result<Value> {
    let (r_type, source, r_icon) =
        reaction_spec(&icon).ok_or_else(|| ZaloError::Api(ZaloApiError::new("Invalid reaction")))?;
    if r_type < 0 {
        return Err(ZaloError::Api(ZaloApiError::new("Invalid reaction")));
    }

    let is_group = matches!(dest.thread_type, ThreadType::Group);
    let path = if is_group { "/api/group/reaction" } else { "/api/message/reaction" };
    let url = client.service_url("reaction", path)?;
    let url = client.make_url(&url, &[], true)?;

    let react_msg = serde_json::json!({
        "rMsg": [{
            "gMsgID": dest.data.msg_id.parse::<i64>().unwrap_or(0),
            "cMsgID": dest.data.cli_msg_id.parse::<i64>().unwrap_or(0),
            "msgType": 1,
        }],
        "rIcon": r_icon,
        "rType": r_type,
        "source": source,
    });

    let client_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let mut params = serde_json::json!({
        "react_list": [{
            "message": react_msg.to_string(),
            "clientId": client_id,
        }],
    });
    if is_group {
        params["grid"] = Value::String(dest.thread_id);
        params["imei"] = Value::String(client.ctx().imei.clone());
    } else {
        params["toid"] = Value::String(dest.thread_id);
    }

    client.post_encrypted(&url, &params).await
}
