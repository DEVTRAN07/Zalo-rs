//! Undo (message deletion) types.

use serde::{Deserialize, Serialize};


/// Content of an undo event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TUndoContent {
    #[serde(rename = "globalMsgId")]
    pub global_msg_id: i64,
    #[serde(rename = "cliMsgId")]
    pub cli_msg_id: i64,
    #[serde(rename = "deleteMsg")]
    pub delete_msg: i64,
    #[serde(rename = "srcId")]
    pub src_id: i64,
    #[serde(rename = "destId")]
    pub dest_id: i64,
}

/// Core undo data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TUndo {
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
    pub content: TUndoContent,
    pub notify: String,
    pub ttl: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub uin: String,
    pub cmd: i64,
    pub st: i64,
    pub at: i64,
    #[serde(rename = "realMsgId")]
    pub real_msg_id: String,
}

/// An undo event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Undo {
    pub data: TUndo,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
    #[serde(rename = "isGroup")]
    pub is_group: bool,
}
