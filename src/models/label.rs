//! Label (chat tag) types.

use serde::{Deserialize, Serialize};

/// Label data attached to a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelData {
    pub id: i64,
    pub text: String,
    #[serde(rename = "textKey")]
    pub text_key: String,
    pub conversations: Vec<String>,
    pub color: String,
    pub offset: i64,
    pub emoji: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
}
