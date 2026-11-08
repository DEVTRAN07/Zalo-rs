//! Auto-reply rule types.

use serde::{Deserialize, Serialize};

/// A single auto-reply rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoReplyItem {
    pub id: i64,
    pub weight: i64,
    pub enable: bool,
    #[serde(rename = "modifiedTime")]
    pub modified_time: i64,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    pub content: String,
    pub scope: AutoReplyScope,
    #[serde(default)]
    pub uids: Option<Vec<String>>,
    #[serde(rename = "ownerId")]
    pub owner_id: i64,
    #[serde(default)]
    pub recurrence: Vec<String>,
    #[serde(rename = "createdTime")]
    pub created_time: i64,
}

/// Scope of an auto-reply rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoReplyScope {
    Everyone = 0,
    Stranger = 1,
    SpecificFriends = 2,
    FriendsExcept = 3,
}
