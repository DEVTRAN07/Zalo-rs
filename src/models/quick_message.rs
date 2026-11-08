//! Quick message types.

use serde::{Deserialize, Serialize};

/// A quick message (saved reply).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickMessage {
    pub id: i64,
    pub keyword: String,
    #[serde(rename = "type")]
    pub type_: i64,
    #[serde(rename = "createdTime")]
    pub created_time: i64,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    pub message: QuickMessageContent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<QuickMessageMedia>,
}

/// Content of a quick message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickMessageContent {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,
}

/// Media attached to a quick message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickMessageMedia {
    pub items: Vec<QuickMessageMediaItem>,
}

/// A media item within a quick message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickMessageMediaItem {
    #[serde(rename = "type")]
    pub type_: i64,
    #[serde(rename = "photoId")]
    pub photo_id: i64,
    pub title: String,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "previewThumb")]
    pub preview_thumb: String,
    #[serde(rename = "rawUrl")]
    pub raw_url: String,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: String,
    #[serde(rename = "normalUrl")]
    pub normal_url: String,
    #[serde(rename = "hdUrl")]
    pub hd_url: String,
}
