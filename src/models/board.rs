//! Group board types: notes, pinned messages, and polls.

use serde::{Deserialize, Serialize};

/// Type of a group board item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardType {
    Note = 1,
    PinnedMessage = 2,
    Poll = 3,
}

/// Detail of a poll on the group board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollDetail {
    pub creator: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub joined: bool,
    pub closed: bool,
    #[serde(rename = "poll_id")]
    pub poll_id: i64,
    #[serde(rename = "allow_multi_choices")]
    pub allow_multi_choices: bool,
    #[serde(rename = "allow_add_new_option")]
    pub allow_add_new_option: bool,
    #[serde(rename = "is_anonymous")]
    pub is_anonymous: bool,
    #[serde(rename = "poll_type")]
    pub poll_type: i64,
    #[serde(rename = "created_time")]
    pub created_time: i64,
    #[serde(rename = "updated_time")]
    pub updated_time: i64,
    #[serde(rename = "expired_time")]
    pub expired_time: i64,
    #[serde(rename = "is_hide_vote_preview")]
    pub is_hide_vote_preview: bool,
    #[serde(rename = "num_vote")]
    pub num_vote: i64,
}

/// A poll option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOption {
    pub content: String,
    pub votes: i64,
    pub voted: bool,
    pub voters: Vec<String>,
    #[serde(rename = "option_id")]
    pub option_id: i64,
}

/// Detail of a note on the group board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteDetail {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: i64,
    pub color: i64,
    pub emoji: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    pub duration: i64,
    pub params: NoteParams,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    #[serde(rename = "editorId")]
    pub editor_id: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "editTime")]
    pub edit_time: i64,
    pub repeat: i64,
}

/// Parameters for a note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteParams {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

/// Detail of a pinned message on the group board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedMessageDetail {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: i64,
    pub color: i64,
    pub emoji: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    pub duration: i64,
    /// `Record<string, unknown>` in TS — kept as `serde_json::Value`.
    pub params: serde_json::Value,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    #[serde(rename = "editorId")]
    pub editor_id: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "editTime")]
    pub edit_time: i64,
    pub repeat: i64,
}
