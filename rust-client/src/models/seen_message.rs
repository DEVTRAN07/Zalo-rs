//! Seen message event types.

use serde::{Deserialize, Serialize};

use crate::models::r#enum::ThreadType;

/// User seen-message data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TUserSeenMessage {
    #[serde(rename = "idTo")]
    pub id_to: String,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "realMsgId")]
    pub real_msg_id: String,
}

/// Group seen-message data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupSeenMessage {
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "seenUids")]
    pub seen_uids: Vec<String>,
}

/// User seen message event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSeenMessage {
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub data: TUserSeenMessage,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
}

/// Group seen message event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSeenMessage {
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub data: TGroupSeenMessage,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
}

/// Union of user and group seen-message events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SeenMessage {
    #[serde(rename = "0")]
    User(UserSeenMessage),
    #[serde(rename = "1")]
    Group(GroupSeenMessage),
}
