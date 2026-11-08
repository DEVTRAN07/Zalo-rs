//! Sticker types.

use serde::{Deserialize, Serialize};

/// Sticker detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerDetail {
    pub id: i64,
    #[serde(rename = "cateId")]
    pub cate_id: i64,
    #[serde(rename = "type")]
    pub type_: i64,
    pub text: String,
    pub uri: String,
    pub fkey: i64,
    pub status: i64,
    #[serde(rename = "stickerUrl")]
    pub sticker_url: String,
    #[serde(rename = "stickerSpriteUrl")]
    pub sticker_sprite_url: String,
    #[serde(rename = "stickerWebpUrl")]
    pub sticker_webp_url: Option<String>,
    #[serde(rename = "totalFrames")]
    pub total_frames: i64,
    pub duration: i64,
    #[serde(rename = "effectId")]
    pub effect_id: i64,
    pub checksum: String,
    pub ext: i64,
    pub source: i64,
    pub fss: serde_json::Value,
    #[serde(rename = "fssInfo")]
    pub fss_info: serde_json::Value,
    pub version: i64,
    #[serde(rename = "extInfo")]
    pub ext_info: serde_json::Value,
}

/// Basic sticker reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerBasic {
    #[serde(rename = "type")]
    pub type_: i64,
    #[serde(rename = "cate_id")]
    pub cate_id: i64,
    #[serde(rename = "sticker_id")]
    pub sticker_id: i64,
}
