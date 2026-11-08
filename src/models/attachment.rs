//! Attachment source types used when sending messages.

use serde::{Deserialize, Serialize};

/// Metadata for an in-memory attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentMetadata {
    #[serde(rename = "totalSize")]
    pub total_size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
}

/// Source of an attachment: either a file path or an in-memory buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachmentSource {
    Path(String),
    InMemory {
        data: Vec<u8>,
        filename: String,
        metadata: AttachmentMetadata,
    },
}
