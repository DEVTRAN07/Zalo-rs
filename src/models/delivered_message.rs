//! Delivered message event types.

use serde::{Deserialize, Serialize};

use crate::models::r#enum::ThreadType;

/// Core delivered-message data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDeliveredMessage {
    #[serde(rename = "msgId")]
    pub msg_id: String,
    pub seen: i64,
    #[serde(rename = "deliveredUids")]
    pub delivered_uids: Vec<String>,
    #[serde(rename = "seenUids")]
    pub seen_uids: Vec<String>,
    #[serde(rename = "realMsgId")]
    pub real_msg_id: String,
    #[serde(rename = "mSTs")]
    pub m_sts: i64,
}

/// Group delivered-message data (extends TDeliveredMessage with group_id).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupDeliveredMessage {
    #[serde(rename = "msgId")]
    pub msg_id: String,
    pub seen: i64,
    #[serde(rename = "deliveredUids")]
    pub delivered_uids: Vec<String>,
    #[serde(rename = "seenUids")]
    pub seen_uids: Vec<String>,
    #[serde(rename = "realMsgId")]
    pub real_msg_id: String,
    #[serde(rename = "mSTs")]
    pub m_sts: i64,
    #[serde(rename = "groupId")]
    pub group_id: String,
}

/// User-to-user delivered message event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeliveredMessage {
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub data: TDeliveredMessage,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
}

/// Group delivered message event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDeliveredMessage {
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub data: TGroupDeliveredMessage,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
}

/// Union of user and group delivered-message events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DeliveredMessage {
    #[serde(rename = "0")]
    User(UserDeliveredMessage),
    #[serde(rename = "1")]
    Group(GroupDeliveredMessage),
}
