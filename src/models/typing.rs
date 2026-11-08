//! Typing event types.

use serde::{Deserialize, Serialize};

use crate::models::r#enum::ThreadType;

/// Core typing data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTyping {
    pub uid: String,
    pub ts: String,
    #[serde(rename = "isPC")]
    pub is_pc: i64,
}

/// Group typing data (adds gid).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupTyping {
    pub uid: String,
    pub ts: String,
    #[serde(rename = "isPC")]
    pub is_pc: i64,
    #[serde(rename = "gid")]
    pub gid: String,
}

/// User typing event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTyping {
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub data: TTyping,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
}

/// Group typing event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTyping {
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub data: TGroupTyping,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
}

/// Union of user and group typing events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Typing {
    #[serde(rename = "0")]
    User(UserTyping),
    #[serde(rename = "1")]
    Group(GroupTyping),
}
