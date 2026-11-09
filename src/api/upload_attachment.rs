use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::sync::oneshot;
use tokio::time::timeout;

use crate::api::client::ApiClient;
use crate::context::UploadCallback;
use crate::core::utils::get_file_extension;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachmentSource {
    Path(String),
    Inline {
        filename: String,
        #[serde(skip)]
        data: Vec<u8>,
        #[serde(default)]
        metadata: Value,
    },
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadAttachmentImageResponse {
    pub normal_url: String,
    pub photo_id: String,
    pub finished: Value,
    pub hd_url: String,
    pub thumb_url: String,
    pub client_file_id: i64,
    pub chunk_id: i64,
    pub file_type: String,
    pub width: u64,
    pub height: u64,
    pub total_size: u64,
    pub hd_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadAttachmentVideoResponse {
    pub finished: Value,
    pub client_file_id: i64,
    pub chunk_id: i64,
    pub file_type: String,
    pub file_url: String,
    pub file_id: String,
    pub checksum: String,
    pub total_size: u64,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadAttachmentFileResponse {
    pub finished: Value,
    pub client_file_id: i64,
    pub chunk_id: i64,
    pub file_type: String,
    pub file_url: String,
    pub file_id: String,
    pub checksum: String,
    pub total_size: u64,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadAttachmentType {
    Image(UploadAttachmentImageResponse),
    Video(UploadAttachmentVideoResponse),
    File(UploadAttachmentFileResponse),
}

pub type UploadAttachmentResponse = Vec<UploadAttachmentType>;

#[derive(Debug, Clone)]
struct RawUploadResponse {
    finished: Value,
    client_file_id: i64,
    chunk_id: i64,
    file_id: Option<String>,
    photo_id: Option<String>,
    normal_url: Option<String>,
    hd_url: Option<String>,
    thumb_url: Option<String>,
}

impl RawUploadResponse {
    fn from_value(value: Value) -> Self {
        Self {
            finished: value.get("finished").cloned().unwrap_or(Value::Null),
            client_file_id: value
                .get("clientFileId")
                .and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
                .unwrap_or(0),
            chunk_id: value
                .get("chunkId")
                .and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
                .unwrap_or(0),
            file_id: value
                .get("fileId")
                .and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| v.as_i64().map(|n| n.to_string()))),
            photo_id: value
                .get("photoId")
                .and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| v.as_i64().map(|n| n.to_string()))),
            normal_url: value.get("normalUrl").and_then(|v| v.as_str()).map(ToString::to_string),
            hd_url: value.get("hdUrl").and_then(|v| v.as_str()).map(ToString::to_string),
            thumb_url: value.get("thumbUrl").and_then(|v| v.as_str()).map(ToString::to_string),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum FileKind {
    Image,
    Video,
    Other,
}

impl FileKind {
    fn url_type(self) -> &'static str {
        match self {
            FileKind::Image => "photo_original/upload",
            FileKind::Video | FileKind::Other => "asyncfile/upload",
        }
    }

}

fn sharefile_setting_u64(settings: &Option<Value>, key: &str, default: u64) -> u64 {
    settings
        .as_ref()
        .and_then(|s| s.get("features").or_else(|| s.get("sharefile")))
        .and_then(|s| if s.get(key).is_some() { Some(s) } else { s.get("sharefile") })
        .and_then(|s| s.get(key))
        .and_then(|v| v.as_u64().or_else(|| v.as_i64().map(|n| n.max(0) as u64)))
        .unwrap_or(default)
}

fn restricted_extensions(settings: &Option<Value>) -> Vec<String> {
    let values = settings
        .as_ref()
        .and_then(|s| s.get("features").or_else(|| s.get("sharefile")))
        .and_then(|s| s.get("sharefile").or(Some(s)))
        .and_then(|s| s.get("restricted_ext_file"))
        .and_then(|v| v.as_array());
    values
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.trim_start_matches('.').to_ascii_lowercase()))
                .collect()
        })
        .unwrap_or_default()
}

fn is_extension_valid(ext: &str, settings: &Option<Value>) -> bool {
    let restricted = restricted_extensions(settings);
    if restricted.is_empty() {
        return true;
    }
    !restricted.iter().any(|e| e.eq_ignore_ascii_case(ext))
}

fn classify_extension(ext: &str) -> FileKind {
    match ext.to_ascii_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "webp" => FileKind::Image,
        "mp4" => FileKind::Video,
        _ => FileKind::Other,
    }
}

fn image_dimensions(data: &[u8]) -> Option<(u64, u64)> {
    if data.starts_with(b"\x89PNG\r\n\x1a\n") && data.len() >= 24 && &data[12..16] == b"IHDR" {
        let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]) as u64;
        let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]) as u64;
        return Some((width, height));
    }

    if data.starts_with(&[0xff, 0xd8]) {
        let mut i = 2usize;
        while i + 9 < data.len() {
            if data[i] != 0xff {
                i += 1;
                continue;
            }
            let marker = data[i + 1];
            i += 2;
            if marker == 0xd8 || marker == 0xd9 {
                continue;
            }
            if i + 2 > data.len() {
                break;
            }
            let len = u16::from_be_bytes([data[i], data[i + 1]]) as usize;
            if len < 2 || i + len > data.len() {
                break;
            }
            if (0xc0..=0xc3).contains(&marker)
                || (0xc5..=0xc7).contains(&marker)
                || (0xc9..=0xcb).contains(&marker)
                || (0xcd..=0xcf).contains(&marker)
            {
                if i + 7 >= data.len() {
                    break;
                }
                let height = u16::from_be_bytes([data[i + 3], data[i + 4]]) as u64;
                let width = u16::from_be_bytes([data[i + 5], data[i + 6]]) as u64;
                if width > 0 && height > 0 {
                    return Some((width, height));
                }
            }
            i += len;
        }
    }

    // WebP: RIFF....WEBPVP8 / VP8L / VP8X
    if data.len() >= 30 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" {
        if &data[12..16] == b"VP8 " && data.len() >= 30 {
            // lossy: starts at 0x14 in some streams; scan for sync code 0x9d012a
            if let Some(pos) = data.windows(3).position(|w| w == [0x9d, 0x01, 0x2a]) {
                let p = pos + 3;
                if p + 4 <= data.len() {
                    let width = u16::from_le_bytes([data[p], data[p + 1]]) & 0x3fff;
                    let height = u16::from_le_bytes([data[p + 2], data[p + 3]]) & 0x3fff;
                    if width > 0 && height > 0 {
                        return Some((width as u64, height as u64));
                    }
                }
            }
        }
        if &data[12..16] == b"VP8L" && data.len() >= 25 && data[20] == 0x2f {
            let bits = u32::from_le_bytes([data[21], data[22], data[23], data[24]]);
            let width = ((bits & 0x3fff) + 1) as u64;
            let height = (((bits >> 14) & 0x3fff) + 1) as u64;
            return Some((width, height));
        }
        if &data[12..16] == b"VP8X" && data.len() >= 30 {
            let width = 1u64
                + (data[24] as u64)
                + ((data[25] as u64) << 8)
                + ((data[26] as u64) << 16);
            let height = 1u64
                + (data[27] as u64)
                + ((data[28] as u64) << 8)
                + ((data[29] as u64) << 16);
            return Some((width, height));
        }
    }

    None
}

async fn file_bytes_and_name(source: &AttachmentSource) -> Result<(Vec<u8>, String)> {
    match source {
        AttachmentSource::Path(path) => {
            let data = fs::read(path).await.map_err(|e| {
                ZaloError::Api(ZaloApiError::new(format!(
                    "Failed to read file {}: {}",
                    path, e
                )))
            })?;
            let filename = Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file")
                .to_string();
            Ok((data, filename))
        }
        AttachmentSource::Inline { filename, data, .. } => Ok((data.clone(), filename.clone())),
    }
}

async fn compute_checksum(source: &AttachmentSource, _total_size: u64) -> Result<String> {
    use md5::{Digest, Md5};
    let mut hasher = Md5::new();
    match source {
        AttachmentSource::Path(path) => {
            let bytes = fs::read(path).await.map_err(|e| {
                ZaloError::Api(ZaloApiError::new(format!("Failed to read file {}: {}", path, e)))
            })?;
            hasher.update(&bytes);
        }
        AttachmentSource::Inline { data, .. } => hasher.update(data),
    }
    Ok(hex::encode(hasher.finalize()))
}

async fn post_chunk(
    client: &ApiClient,
    request_url: &str,
    chunk: Vec<u8>,
    filename: &str,
) -> Result<RawUploadResponse> {
    let form = reqwest::multipart::Form::new().part(
        "chunkContent",
        reqwest::multipart::Part::bytes(chunk)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .unwrap_or_else(|_| reqwest::multipart::Part::bytes(Vec::new())),
    );

    let base = client.ctx_to_base();
    let http = crate::http::get_http(&base).await?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ORIGIN,
        reqwest::header::HeaderValue::from_static("https://chat.zalo.me"),
    );
    headers.insert(
        reqwest::header::REFERER,
        reqwest::header::HeaderValue::from_static("https://chat.zalo.me/"),
    );
    if let Ok(ua) = reqwest::header::HeaderValue::from_str(&client.ctx().user_agent) {
        headers.insert(reqwest::header::USER_AGENT, ua);
    }

    let resp = http
        .client
        .post(request_url)
        .headers(headers)
        .multipart(form)
        .send()
        .await
        .map_err(ZaloError::Http)?;

    let status = resp.status();
    if !status.is_success() {
        return Err(ZaloError::Api(ZaloApiError::new(format!(
            "Upload failed with status {}",
            status
        ))));
    }

    // Upload responses are typically plain JSON; attempt plain parse first, fall back to encrypted resolver.
    let body = resp.bytes().await.map_err(ZaloError::Http)?;
    if let Ok(value) = serde_json::from_slice::<Value>(&body) {
        if let Some(error_code) = value.get("error_code").and_then(|v| v.as_i64()) {
            if error_code != 0 {
                let msg = value
                    .get("error_message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Upload error")
                    .to_string();
                return Err(ZaloError::Api(ZaloApiError::with_code(msg, error_code)));
            }
        }
        let data = value.get("data").cloned().unwrap_or(value);
        if let Some(error_code) = data.get("error_code").and_then(|v| v.as_i64()) {
            if error_code != 0 {
                let msg = data
                    .get("error_message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Upload error")
                    .to_string();
                return Err(ZaloError::Api(ZaloApiError::with_code(msg, error_code)));
            }
        }
        return Ok(RawUploadResponse::from_value(data));
    }

    Err(ZaloError::Api(ZaloApiError::new(
        "Failed to parse upload response",
    )))
}

pub async fn upload_attachment(
    client: &ApiClient,
    sources: Vec<AttachmentSource>,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<UploadAttachmentResponse> {
    upload_attachment_internal(client, sources, thread_id, thread_type).await
}

pub async fn upload_attachment_sources(
    client: &ApiClient,
    sources: Vec<AttachmentSource>,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<UploadAttachmentResponse> {
    upload_attachment_internal(client, sources, thread_id, thread_type).await
}

async fn upload_attachment_internal(
    client: &ApiClient,
    sources: Vec<AttachmentSource>,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<UploadAttachmentResponse> {
    if sources.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing sources")));
    }
    if thread_id.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing threadId")));
    }

    let settings = client.ctx().settings.clone();
    let max_file = sharefile_setting_u64(&settings, "max_file", 10) as usize;
    if sources.len() > max_file {
        return Err(ZaloError::Api(ZaloApiError::new(format!(
            "Exceed maximum file of {}",
            max_file
        ))));
    }

    let chunk_size = sharefile_setting_u64(&settings, "chunk_size_file", 1_048_576) as usize;
    let max_size_mb = sharefile_setting_u64(&settings, "max_size_share_file_v3", 100);
    let max_size_bytes = max_size_mb.saturating_mul(1024 * 1024);

    let is_group = matches!(thread_type, ThreadType::Group);
    let file_base = client.service_url("file", "/api")?;
    let type_param = if is_group { "11" } else { "2" };
    let mut client_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let mut results: UploadAttachmentResponse = Vec::with_capacity(sources.len());

    for source in sources {
        // Validate existence for paths early.
        if let AttachmentSource::Path(path) = &source {
            if !Path::new(path).exists() {
                return Err(ZaloError::Api(ZaloApiError::new("File not found")));
            }
        }

        let (bytes, file_name) = file_bytes_and_name(&source).await?;
        let ext = get_file_extension(if let AttachmentSource::Path(p) = &source { p } else { &file_name })
            .trim_start_matches('.')
            .to_ascii_lowercase();
        if !is_extension_valid(&ext, &settings) {
            return Err(ZaloError::Api(ZaloApiError::new(format!(
                "File extension \"{}\" is not allowed",
                ext
            ))));
        }

        let total_size = bytes.len() as u64;
        if max_size_bytes > 0 && total_size > max_size_bytes {
            return Err(ZaloError::Api(ZaloApiError::new(format!(
                "File {} size exceed maximum size of {}MB",
                file_name, max_size_mb
            ))));
        }

        let kind = classify_extension(&ext);
        let (width, height) = if matches!(kind, FileKind::Image) {
            image_dimensions(&bytes).unwrap_or((0, 0))
        } else {
            (0, 0)
        };

        let total_chunk = std::cmp::max(1, ((total_size as usize) + chunk_size - 1) / chunk_size);
        let mut params = serde_json::Map::new();
        if is_group {
            params.insert("grid".to_string(), Value::String(thread_id.to_string()));
        } else {
            params.insert("toid".to_string(), Value::String(thread_id.to_string()));
        }
        params.insert("totalChunk".to_string(), Value::from(total_chunk as i64));
        params.insert("fileName".to_string(), Value::String(file_name.clone()));
        params.insert("clientId".to_string(), Value::from(client_id));
        client_id += 1;
        params.insert("totalSize".to_string(), Value::from(total_size as i64));
        params.insert("imei".to_string(), Value::String(client.ctx().imei.clone()));
        params.insert("isE2EE".to_string(), Value::from(0));
        params.insert("jxl".to_string(), Value::from(0));
        params.insert("chunkId".to_string(), Value::from(1));

        let url_path = format!(
            "{}/{}/{}",
            file_base.trim_end_matches('/'),
            if is_group { "group" } else { "message" },
            kind.url_type()
        );

        let mut last_raw: Option<RawUploadResponse> = None;
        for (idx, chunk) in bytes.chunks(chunk_size).enumerate() {
            params.insert("chunkId".to_string(), Value::from((idx + 1) as i64));
            let params_value = Value::Object(params.clone());
            let encrypted = client.encode_aes(&params_value.to_string()).ok_or_else(|| {
                ZaloError::Api(ZaloApiError::new("Failed to encrypt message"))
            })?;

            let request_url = format!("{}?type={}&params={}", url_path, type_param, urlencoding::encode(&encrypted));
            let raw = post_chunk(client, &request_url, chunk.to_vec(), &file_name).await?;
            last_raw = Some(raw);
        }

        let raw = last_raw.ok_or_else(|| ZaloError::Api(ZaloApiError::new("Empty upload response")))?;

        let valid = raw
            .file_id
            .as_deref()
            .map(|id| id != "-1")
            .unwrap_or(true)
            && raw
                .photo_id
                .as_deref()
                .map(|id| id != "-1")
                .unwrap_or(true);

        if !valid {
            return Err(ZaloError::Api(ZaloApiError::new("Upload rejected by server")));
        }

        match kind {
            FileKind::Image => {
                results.push(UploadAttachmentType::Image(UploadAttachmentImageResponse {
                    normal_url: raw.normal_url.unwrap_or_default(),
                    photo_id: raw.photo_id.unwrap_or_default(),
                    finished: raw.finished,
                    hd_url: raw.hd_url.clone().unwrap_or_default(),
                    thumb_url: raw.thumb_url.unwrap_or_default(),
                    client_file_id: raw.client_file_id,
                    chunk_id: raw.chunk_id,
                    file_type: "image".to_string(),
                    width,
                    height,
                    total_size,
                    hd_size: total_size,
                }));
            }
            FileKind::Video | FileKind::Other => {
                let file_id = raw.file_id.clone().unwrap_or_default();
                let mut ws_data: Option<Value> = None;

                if !file_id.is_empty() {
                    let (tx, rx) = oneshot::channel::<Value>();
                    let tx = Arc::new(std::sync::Mutex::new(Some(tx)));
                    let callback: UploadCallback = Arc::new(move |data| {
                        let tx = tx.clone();
                        Box::pin(async move {
                            if let Some(sender) = tx.lock().unwrap_or_else(|e| e.into_inner()).take() {
                                let _ = sender.send(data);
                            }
                        }) as Pin<Box<dyn Future<Output = ()> + Send>>
                    });

                    if let Ok(mut map) = client.ctx().upload_callbacks.lock() {
                        map.insert(file_id.clone(), callback);
                    }

                    ws_data = timeout(Duration::from_secs(120), rx).await.ok().and_then(|r| r.ok());

                    if let Ok(mut map) = client.ctx().upload_callbacks.lock() {
                        map.remove(&file_id);
                    }
                }

                let checksum = compute_checksum(&source, total_size).await.unwrap_or_default();
                let file_url = ws_data
                    .as_ref()
                    .and_then(|d| d.get("fileUrl").or_else(|| d.get("file_url")))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let resolved_file_id = ws_data
                    .as_ref()
                    .and_then(|d| d.get("fileId").or_else(|| d.get("file_id")))
                    .and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| v.as_i64().map(|n| n.to_string())))
                    .unwrap_or(file_id);

                if matches!(kind, FileKind::Video) {
                    results.push(UploadAttachmentType::Video(UploadAttachmentVideoResponse {
                        finished: raw.finished,
                        client_file_id: raw.client_file_id,
                        chunk_id: raw.chunk_id,
                        file_type: "video".to_string(),
                        file_url,
                        file_id: resolved_file_id,
                        checksum,
                        total_size,
                        file_name: file_name.clone(),
                    }));
                } else {
                    results.push(UploadAttachmentType::File(UploadAttachmentFileResponse {
                        finished: raw.finished,
                        client_file_id: raw.client_file_id,
                        chunk_id: raw.chunk_id,
                        file_type: "others".to_string(),
                        file_url,
                        file_id: resolved_file_id,
                        checksum,
                        total_size,
                        file_name: file_name.clone(),
                    }));
                }
            }
        }
    }

    Ok(results)
}
