use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;
use tokio::fs;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

/// Source for a product photo upload (path or inline bytes).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductPhotoSource {
    Path(String),
    Inline {
        filename: Option<String>,
        #[serde(skip)]
        data: Vec<u8>,
        #[serde(default)]
        metadata: Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadProductPhotoPayload {
    pub file: Option<ProductPhotoSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadProductPhotoResponse {
    #[serde(default)]
    pub normal_url: String,
    #[serde(default)]
    pub photo_id: String,
    #[serde(default)]
    pub finished: i64,
    #[serde(default)]
    pub hd_url: String,
    #[serde(default)]
    pub thumb_url: String,
    #[serde(default)]
    pub client_file_id: i64,
    #[serde(default)]
    pub chunk_id: i64,
}

async fn file_bytes_and_name(source: &ProductPhotoSource) -> Result<(Vec<u8>, String, u64)> {
    match source {
        ProductPhotoSource::Path(path) => {
            let data = fs::read(path).await.map_err(|e| {
                ZaloError::Api(ZaloApiError::new(format!(
                    "Failed to read file {}: {}",
                    path, e
                )))
            })?;
            let filename = Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("photo.jpg")
                .to_string();
            let total_size = data.len() as u64;
            Ok((data, filename, total_size))
        }
        ProductPhotoSource::Inline { filename, data, .. } => {
            let name = filename
                .clone()
                .unwrap_or_else(|| "photo.jpg".to_string());
            let total_size = data.len() as u64;
            Ok((data.clone(), name, total_size))
        }
    }
}

pub async fn upload_product_photo(
    client: &ApiClient,
    payload: UploadProductPhotoPayload,
) -> Result<UploadProductPhotoResponse> {
    let source = payload
        .file
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Missing file source")))?;

    let (data, filename, total_size) = file_bytes_and_name(&source).await?;

    let url = client.service_url("file", "/api/product/upload/photo")?;

    let client_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let toid = client
        .ctx()
        .login_info
        .extra
        .get("send2me_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_default();

    let params = json!({
        "totalChunk": 1,
        "fileName": format!("Base64_Img_Picker_{}.jpg", client_id),
        "clientId": client_id,
        "totalSize": total_size,
        "imei": client.ctx().imei.clone(),
        "chunkId": 1,
        "toid": toid,
        "featureId": 1,
    });

    let encrypted = client.encode_aes(&params.to_string()).ok_or_else(|| {
        ZaloError::Api(ZaloApiError::new("Failed to encrypt params"))
    })?;

    let request_url = format!(
        "{}?type=1&params={}",
        url.trim_end_matches('/'),
        urlencoding::encode(&encrypted)
    );

    let form = reqwest::multipart::Form::new().part(
        "chunkContent",
        reqwest::multipart::Part::bytes(data)
            .file_name(filename)
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
        .post(&request_url)
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

    let body = resp.bytes().await.map_err(ZaloError::Http)?;
    let value: Value = serde_json::from_slice(&body).map_err(|e| {
        ZaloError::Api(ZaloApiError::new(format!(
            "Failed to parse upload response: {}",
            e
        )))
    })?;

    let data_val = value.get("data").cloned().unwrap_or(value);

    if let Some(error_code) = data_val.get("error_code").and_then(|v| v.as_i64()) {
        if error_code != 0 {
            let msg = data_val
                .get("error_message")
                .and_then(|v| v.as_str())
                .unwrap_or("Upload error")
                .to_string();
            return Err(ZaloError::Api(ZaloApiError::with_code(msg, error_code)));
        }
    }

    let normal_url = data_val
        .get("normalUrl")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let photo_id = data_val
        .get("photoId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let finished = data_val
        .get("finished")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let hd_url = data_val
        .get("hdUrl")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let thumb_url = data_val
        .get("thumbUrl")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let client_file_id = data_val
        .get("clientFileId")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let chunk_id = data_val
        .get("chunkId")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    Ok(UploadProductPhotoResponse {
        normal_url,
        photo_id,
        finished,
        hd_url,
        thumb_url,
        client_file_id,
        chunk_id,
    })
}
