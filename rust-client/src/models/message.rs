//! Message types: raw message data and wrapped message events.

use serde::{Deserialize, Serialize};

use crate::models::r#enum::ThreadType;

/// Attachment content within a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TAttachmentContent {
    pub title: String,
    pub description: String,
    pub href: String,
    pub thumb: String,
    pub childnumber: i64,
    pub action: String,
    pub params: String,
    pub type_: String,
}

/// Other / unknown content (a map).
pub type TOtherContent = serde_json::Map<String, serde_json::Value>;

/// Core message data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMessage {
    #[serde(rename = "actionId")]
    pub action_id: String,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "cliMsgId")]
    pub cli_msg_id: String,
    #[serde(rename = "msgType")]
    pub msg_type: String,
    #[serde(rename = "uidFrom")]
    pub uid_from: String,
    #[serde(rename = "idTo")]
    pub id_to: String,
    #[serde(rename = "dName")]
    pub d_name: String,
    pub ts: String,
    pub status: i64,
    pub content: serde_json::Value,
    pub notify: String,
    pub ttl: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub uin: String,
    #[serde(rename = "topOut")]
    pub top_out: String,
    #[serde(rename = "topOutTimeOut")]
    pub top_out_time_out: String,
    #[serde(rename = "topOutImprTimeOut")]
    pub top_out_impr_time_out: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property_ext: Option<MessagePropertyExt>,
    #[serde(rename = "paramsExt")]
    pub params_ext: MessageParamsExt,
    pub cmd: i64,
    pub st: i64,
    pub at: i64,
    #[serde(rename = "realMsgId")]
    pub real_msg_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<TQuote>,
}

/// Property extension within a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePropertyExt {
    pub color: i64,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_: i64,
    #[serde(rename = "subType")]
    pub sub_type: i64,
    pub ext: String,
}

/// Parameters extension within a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageParamsExt {
    #[serde(rename = "countUnread")]
    pub count_unread: i64,
    #[serde(rename = "containType")]
    pub contain_type: i64,
    #[serde(rename = "platformType")]
    pub platform_type: i64,
}

/// A quoted message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TQuote {
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "cliMsgId")]
    pub cli_msg_id: i64,
    #[serde(rename = "globalMsgId")]
    pub global_msg_id: i64,
    #[serde(rename = "cliMsgType")]
    pub cli_msg_type: i64,
    pub ts: i64,
    pub msg: String,
    pub attach: String,
    #[serde(rename = "fromD")]
    pub from_d: String,
    pub ttl: i64,
}

/// A mention within a group message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMention {
    pub uid: String,
    pub pos: i64,
    pub len: i64,
    pub type_: i64,
}

/// Group message data (message + mentions).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupMessage {
    #[serde(flatten)]
    pub base: TMessage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<TMention>>,
}

/// User-to-user message event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub data: TMessage,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
}

/// Group message event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMessage {
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub data: TGroupMessage,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
}

/// Union of user and group message events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "0")]
    User(UserMessage),
    #[serde(rename = "1")]
    Group(GroupMessage),
}
