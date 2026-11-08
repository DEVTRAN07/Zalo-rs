//! General purpose helpers: file metadata, formatting, event decoding.

use std::fs;
use std::io::Read;
use std::path::Path;

use aes_gcm::{AesGcm, KeyInit};
use aes_gcm::aead::Aead;
use aes_gcm::aes::Aes128;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::{Datelike, Timelike};
use flate2::read::ZlibDecoder;
use generic_array::GenericArray;
use generic_array::typenum::U12;
use md5::{Digest, Md5};
use serde_json::Map;
use unicode_normalization::UnicodeNormalization;

use crate::errors::ZaloApiError;
use crate::models::attachment::AttachmentSource;
use crate::models::friend_event::FriendEventType;
use crate::models::group_event::GroupEventType;

/// Always `false` on Rust (no Bun runtime).
pub const IS_BUN: bool = false;

/// Check if a JSON map owns a key.
pub fn has_own(obj: &Map<String, serde_json::Value>, key: &str) -> bool {
    obj.contains_key(key)
}

/// Remove keys with `null` values from a JSON map (Rust equivalent of removing
/// `undefined` keys).
pub fn remove_undefined_keys(obj: &mut Map<String, serde_json::Value>) {
    obj.retain(|_, v| !v.is_null());
}

/// Decode a Base64 string into a byte buffer.
pub fn decode_base64_to_buffer(data: &str) -> Vec<u8> {
    BASE64.decode(data).unwrap_or_default()
}

/// Decode a UTF-8 byte array into a string, or `None` on invalid UTF-8.
pub fn decode_unit8_array(data: &[u8]) -> Option<String> {
    std::str::from_utf8(data).ok().map(String::from)
}

/// A decoded websocket event payload.
#[derive(Debug, Clone)]
pub enum DecodedEventData {
    Object(Map<String, serde_json::Value>),
    Value(serde_json::Value),
}

/// Decodes a websocket event payload, decrypting/decompressing it when needed.
///
/// # Errors
/// Returns a [`ZaloApiError`] if the payload is invalid, the cipher key is
/// missing for encrypted types, or decryption/decompression fails.
pub async fn decode_event_data(
    parsed: &Map<String, serde_json::Value>,
    cipher_key: Option<&str>,
) -> Result<Option<DecodedEventData>, ZaloApiError> {
    let data = parsed
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ZaloApiError::new("Invalid data, expected string"))?;

    let encrypt = parsed
        .get("encrypt")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| ZaloApiError::new("Invalid encrypt type, expected number"))?;

    if !(0..=3).contains(&encrypt) {
        return Err(ZaloApiError::new(format!(
            "Invalid encrypt type, expected 0-3 but got {}",
            encrypt
        )));
    }

    if encrypt == 0 {
        let value: serde_json::Value = serde_json::from_str(data)
            .map_err(|e| ZaloApiError::new(format!("Failed to parse JSON: {}", e)))?;
        return Ok(Some(DecodedEventData::Value(value)));
    }

    let raw_data: String = if encrypt == 1 {
        data.to_string()
    } else {
        urlencoding::decode(data)
            .map_err(|e| ZaloApiError::new(format!("Failed to URL-decode data: {}", e)))?
            .into_owned()
    };

    let decoded_buffer = BASE64.decode(raw_data).map_err(|e| {
        ZaloApiError::new(format!("Failed to base64-decode data: {}", e))
    })?;

    let mut decrypted_buffer = decoded_buffer.clone();
    if encrypt != 1 {
        let key_b64 = cipher_key.ok_or_else(|| {
            ZaloApiError::new("Invalid data length or missing cipher key")
        })?;
        if decoded_buffer.len() < 48 {
            return Err(ZaloApiError::new("Invalid data length or missing cipher key"));
        }

        let key_bytes = BASE64.decode(key_b64).map_err(|e| {
            ZaloApiError::new(format!("Failed to decode cipher key: {}", e))
        })?;
        if key_bytes.len() != 16 {
            return Err(ZaloApiError::new("Cipher key must be 16 bytes"));
        }

        let cipher = AesGcm::<Aes128, U12>::new_from_slice(&key_bytes).map_err(|e| ZaloApiError::new(format!("Failed to create cipher: {}", e)))?;
        let nonce = GenericArray::<u8, U12>::from_slice(&decoded_buffer[0..12]);
        let tag = &decoded_buffer[12..28];
        let ciphertext = &decoded_buffer[28..];

        let mut buf_with_tag = Vec::with_capacity(ciphertext.len() + 16);
        buf_with_tag.extend_from_slice(ciphertext);
        buf_with_tag.extend_from_slice(tag);

        decrypted_buffer = cipher
            .decrypt(nonce, buf_with_tag.as_slice())
            .map_err(|e| ZaloApiError::new(format!("AES-GCM decryption failed: {}", e)))?;
    }

    let decompressed_buffer = if encrypt == 3 {
        decrypted_buffer
    } else {
        let mut decoder = ZlibDecoder::new(&decrypted_buffer[..]);
        let mut out = Vec::new();
        decoder.read_to_end(&mut out).map_err(|e| {
            ZaloApiError::new(format!("Failed to decompress data: {}", e))
        })?;
        out
    };

    let decoded_data = String::from_utf8(decompressed_buffer).ok();
    let decoded_data = decoded_data.filter(|s| !s.is_empty());

    match decoded_data {
        Some(s) => {
            let value: serde_json::Value = serde_json::from_str(&s).map_err(|e| {
                ZaloApiError::new(format!("Failed to parse decoded JSON: {}", e))
            })?;
            Ok(Some(DecodedEventData::Value(value)))
        }
        None => Ok(None),
    }
}

/// MD5 of a (possibly large) file, computed by streaming the source in 2MB
/// chunks so memory usage stays constant regardless of file size.
pub async fn get_md5_large_file_object(
    source: &AttachmentSource,
    _file_size: u64,
) -> Result<(u64, String), ZaloApiError> {
    const CHUNK_SIZE: usize = 2_097_152; // 2MB
    let mut hasher = Md5::new();
    let mut current_chunk: u64 = 0;

    match source {
        AttachmentSource::Path(path) => {
            let mut file = fs::File::open(path).map_err(|e| {
                ZaloApiError::new(format!("Failed to open file: {}", e))
            })?;
            let mut buffer = [0u8; CHUNK_SIZE];
            loop {
                let bytes_read = file.read(&mut buffer).map_err(|e| {
                    ZaloApiError::new(format!("Failed to read file: {}", e))
                })?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
                current_chunk += 1;
            }
        }
        AttachmentSource::InMemory { data, .. } => {
            for offset in (0..data.len()).step_by(CHUNK_SIZE) {
                let end = (offset + CHUNK_SIZE).min(data.len());
                hasher.update(&data[offset..end]);
                current_chunk += 1;
            }
        }
    }

    let digest = hex::encode(hasher.finalize());
    Ok((current_chunk, digest))
}

/// Image metadata returned by a metadata getter.
#[derive(Debug, Clone)]
pub struct ImageMetaData {
    pub file_name: String,
    pub total_size: u64,
    pub width: u64,
    pub height: u64,
}

/// Get image metadata via a context-provided getter callback.
pub async fn get_image_meta_data<F, Fut>(
    file_path: &str,
    getter: F,
) -> Result<ImageMetaData, ZaloApiError>
where
    F: FnOnce(&str) -> Fut,
    Fut: std::future::Future<Output = Option<ImageMetaData>>,
{
    let image_data = getter(file_path).await.ok_or_else(|| {
        ZaloApiError::new("Failed to get image metadata")
    })?;

    Ok(ImageMetaData {
        file_name: get_file_name(file_path).to_string(),
        total_size: image_data.total_size,
        width: image_data.width,
        height: image_data.height,
    })
}

/// Get GIF metadata via a context-provided getter callback.
pub async fn get_gif_meta_data<F, Fut>(
    file_path: &str,
    getter: F,
) -> Result<ImageMetaData, ZaloApiError>
where
    F: FnOnce(&str) -> Fut,
    Fut: std::future::Future<Output = Option<ImageMetaData>>,
{
    let gif_data = getter(file_path).await.ok_or_else(|| {
        ZaloApiError::new("Failed to get gif metadata")
    })?;

    Ok(ImageMetaData {
        file_name: Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(file_path)
            .to_string(),
        total_size: gif_data.total_size,
        width: gif_data.width,
        height: gif_data.height,
    })
}

/// Get the size of a file in bytes.
pub async fn get_file_size(file_path: &str) -> Result<u64, ZaloApiError> {
    let metadata = fs::metadata(file_path).map_err(|e| {
        ZaloApiError::new(format!("Failed to stat file: {}", e))
    })?;
    Ok(metadata.len())
}

/// Get the extension of a file path (without the leading dot).
pub fn get_file_extension(file_path: &str) -> String {
    Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string()
}

/// Get the file name from a path.
pub fn get_file_name(file_path: &str) -> &str {
    Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(file_path)
}

/// Left-pad a string to a given length.
pub fn str_pad_left(value: &str, pad: &str, length: usize) -> String {
    let s = value.to_string();
    if s.len() == length {
        return s;
    }
    if s.len() > length {
        return s[s.len() - length..].to_string();
    }
    let repeat_count = length - s.len();
    pad.repeat(repeat_count) + &s
}

/// Format a timestamp according to a format string.
pub fn format_time(format: &str, timestamp: i64) -> String {
    let date = chrono::DateTime::from_timestamp_millis(timestamp)
        .unwrap_or_else(|| chrono::Utc::now());

    if format.contains("%H") || format.contains("%d") {
        let mut result = format.to_string();
        result = result.replace("%H", &format!("{:02}", date.hour()));
        result = result.replace("%M", &format!("{:02}", date.minute()));
        result = result.replace("%S", &format!("{:02}", date.second()));
        result = result.replace("%d", &format!("{:02}", date.day()));
        result = result.replace("%m", &format!("{:02}", date.month()));
        result = result.replace("%Y", &date.year().to_string());
        return result;
    }

    date.format("%d/%m/%Y %H:%M:%S").to_string()
}

/// Get a full time string from a millisecond timestamp.
pub fn get_full_time_from_millisecond(timestamp: i64) -> String {
    let date = chrono::DateTime::from_timestamp_millis(timestamp)
        .unwrap_or_else(|| chrono::Utc::now());

    format!(
        "{:02}:{:02} {:02}/{:02}/{}",
        date.hour(),
        date.minute(),
        date.day(),
        date.month(),
        date.year()
    )
}

/// Map a client message type string to a numeric code.
pub fn get_client_message_type(msg_type: &str) -> i64 {
    match msg_type {
        "webchat" => 1,
        "chat.voice" => 31,
        "chat.photo" => 32,
        "chat.sticker" => 36,
        "chat.doodle" => 37,
        "chat.recommended" => 38,
        "chat.link" => 38,
        "chat.video.msg" => 44,
        "share.file" => 46,
        "chat.gif" => 49,
        "chat.location.new" => 43,
        _ => 1,
    }
}

/// Map a group event action string to a [`GroupEventType`].
pub fn get_group_event_type(act: &str) -> GroupEventType {
    match act {
        "join_request" => GroupEventType::JoinRequest,
        "join" => GroupEventType::Join,
        "leave" => GroupEventType::Leave,
        "remove_member" => GroupEventType::RemoveMember,
        "block_member" => GroupEventType::BlockMember,
        "update_setting" => GroupEventType::UpdateSetting,
        "update" => GroupEventType::Update,
        "new_link" => GroupEventType::NewLink,
        "add_admin" => GroupEventType::AddAdmin,
        "remove_admin" => GroupEventType::RemoveAdmin,
        "new_pin_topic" => GroupEventType::NewPinTopic,
        "update_pin_topic" => GroupEventType::UpdatePinTopic,
        "update_board" => GroupEventType::UpdateBoard,
        "remove_board" => GroupEventType::RemoveBoard,
        "reorder_pin_topic" => GroupEventType::ReorderPinTopic,
        "unpin_topic" => GroupEventType::UnpinTopic,
        "remove_topic" => GroupEventType::RemoveTopic,
        "accept_remind" => GroupEventType::AcceptRemind,
        "reject_remind" => GroupEventType::RejectRemind,
        "remind_topic" => GroupEventType::RemindTopic,
        "update_avatar" => GroupEventType::UpdateAvatar,
        _ => GroupEventType::Unknown,
    }
}

/// Map a friend event action string to a [`FriendEventType`].
pub fn get_friend_event_type(act: &str) -> FriendEventType {
    match act {
        "add" => FriendEventType::Add,
        "remove" => FriendEventType::Remove,
        "block" => FriendEventType::Block,
        "unblock" => FriendEventType::Unblock,
        "block_call" => FriendEventType::BlockCall,
        "unblock_call" => FriendEventType::UnblockCall,
        "req_v2" => FriendEventType::Request,
        "reject" => FriendEventType::RejectRequest,
        "undo_req" => FriendEventType::UndoRequest,
        "seen_fr_req" => FriendEventType::SeenFriendRequest,
        "pin_unpin" => FriendEventType::PinUnpin,
        "pin_create" => FriendEventType::PinCreate,
        _ => FriendEventType::Unknown,
    }
}

/// Convert a hex color code (`#RRGGBB` / `#AARRGGBB`) into the negative color
/// number used by the Zalo API.
pub fn hex_to_negative_color(hex: &str) -> i64 {
    let hex = if hex.starts_with('#') {
        hex.to_string()
    } else {
        format!("#{}", hex)
    };
    let hex_value: String = if hex[1..].len() == 6 {
        format!("FF{}", &hex[1..])
    } else {
        hex[1..].to_string()
    };
    let decimal = i64::from_str_radix(&hex_value, 16).unwrap_or(0);
    if decimal > 0x7fffffff {
        decimal - 4294967296
    } else {
        decimal
    }
}

/// Convert a negative color number from the Zalo API back into a hex color.
pub fn negative_color_to_hex(negative_color: i64) -> String {
    let positive_color = negative_color + 4294967296;
    format!("#{:06x}", positive_color & 0xffffff)
}

/// Normalize a bank account holder name (uppercase, accent-free, A-Z0-9 only).
pub fn normalize_holder_name(input: Option<&str>) -> Option<String> {
    let input = input?;
    let normalized: String = input
        .nfd()
        .filter(|c| !(*c as u32 >= 0x0300 && *c as u32 <= 0x036f))
        .collect();
    let normalized = normalized.replace(['\u{0110}', '\u{0111}'], "d");
    let normalized = normalized.to_uppercase();
    let normalized: String = normalized
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect();
    let normalized = normalized.trim().to_string();

    if normalized.len() >= 5 {
        Some(normalized)
    } else {
        None
    }
}
