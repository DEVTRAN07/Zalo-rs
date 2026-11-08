//! Reminder types.

use serde::{Deserialize, Serialize};

/// Reminder repeat mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReminderRepeatMode {
    None = 0,
    Daily = 1,
    Weekly = 2,
    Monthly = 3,
}

/// A user reminder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderUser {
    #[serde(rename = "creatorUid")]
    pub creator_uid: String,
    #[serde(rename = "toUid")]
    pub to_uid: String,
    pub emoji: String,
    pub color: i64,
    #[serde(rename = "reminderId")]
    pub reminder_id: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    pub repeat: ReminderRepeatMode,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "editTime")]
    pub edit_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    pub params: ReminderParams,
    #[serde(rename = "type")]
    pub type_: i64,
}

/// Parameters for a reminder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderParams {
    pub title: String,
    #[serde(rename = "setTitle")]
    pub set_title: bool,
}

/// A group reminder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderGroup {
    #[serde(rename = "editorId")]
    pub editor_id: String,
    pub emoji: String,
    pub color: i64,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    #[serde(rename = "editTime")]
    pub edit_time: i64,
    #[serde(rename = "eventType")]
    pub event_type: i64,
    #[serde(rename = "responseMem")]
    pub response_mem: ReminderResponseMem,
    pub params: ReminderParams,
    #[serde(rename = "type")]
    pub type_: i64,
    pub duration: i64,
    #[serde(rename = "repeatInfo")]
    pub repeat_info: Option<ReminderRepeatInfo>,
    #[serde(rename = "repeatData")]
    pub repeat_data: Vec<serde_json::Value>,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    pub repeat: ReminderRepeatMode,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    pub id: String,
}

/// Response member counts for a group reminder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderResponseMem {
    #[serde(rename = "rejectMember")]
    pub reject_member: i64,
    #[serde(rename = "myResp")]
    pub my_resp: i64,
    #[serde(rename = "acceptMember")]
    pub accept_member: i64,
}

/// Repeat info for a group reminder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderRepeatInfo {
    #[serde(rename = "list_ts")]
    pub list_ts: Vec<serde_json::Value>,
}
