use serde::{Deserialize, Serialize};
use serde_json::Value;
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
pub struct UploadProductPhotoPayload {
    pub file: Option<ProductPhotoSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UploadProductPhotoResponse {
    #[serde(rename = "normalUrl", default)]
    pub normal_url: String,
    #[serde(rename = "photoId", default)]
    pub photo_id: String,
    #[serde(default)]
    pub finished: i64,
    #[serde(rename = "hdUrl", default)]
    pub hd_url: String,
    #[serde(rename = "thumbUrl", default)]
    pub thumb_url: String,
    #[serde(rename = "clientFileId", default)]
    pub client_file_id: i64,
    #[serde(rename = "chunkId", default)]
    pub chunk_id: i64,
}

pub async fn upload_product_photo(
    client: &ApiClient,
    payload: UploadProductPhotoPayload,
) -> Result<UploadProductPhotoResponse> {
    let url = client.service_url("catalog", "/api/prodcatalog/uploadProductPhoto")?;

    let mut form = reqwest::multipart::Form::new()
        .text("imei", client.ctx().imei.clone());

    if let Some(source) = &payload.file {
        match source {
            ProductPhotoSource::Path(path) => {
                let data = fs::read(path).await.map_err(|e| {
                    ZaloError::Api(ZaloApiError::new(format!("Failed to read file {}: {}", path, e)))
                })?;
                let filename = Path::new(path).file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("photo.jpg")
                    .to_string();
                form = form.part("file",
                    reqwest::multipart::Part::bytes(data)
                        .file_name(filename)
                        .mime_str("image/jpeg").unwrap());
            }
            ProductPhotoSource::Inline { filename, data, .. } => {
                form = form.part("file",
                    reqwest::multipart::Part::bytes(data.clone())
                        .file_name(filename.clone().unwrap_or_else(|| "photo.jpg".to_string()))
                        .mime_str("image/jpeg").unwrap());
            }
        }
    }

    let base = client.ctx_to_base();
    let http = crate::http::get_http(&base).await?;
    let client_req = http.client.post(url).multipart(form);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::ORIGIN, reqwest::header::HeaderValue::from_static("https://chat.zalo.me"));
    headers.insert(reqwest::header::REFERER, reqwest::header::HeaderValue::from_static("https://chat.zalo.me/"));
    if let Ok(ua) = reqwest::header::HeaderValue::from_str(&client.ctx().user_agent) {
        headers.insert(reqwest::header::USER_AGENT, ua);
    }

    let resp = client_req.headers(headers).send().await.map_err(ZaloError::Http)?;

    if !resp.status().is_success() {
        return Err(ZaloError::Api(ZaloApiError::new(format!(
            "Upload failed with status {}", resp.status()
        ))));
    }

    let json: Value = resp.json().await.map_err(ZaloError::Http)?;

    Ok(UploadProductPhotoResponse {
        normal_url: json.get("normalUrl").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        photo_id: json.get("photoId").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        finished: json.get("finished").and_then(|v| v.as_i64()).unwrap_or(0),
        hd_url: json.get("hdUrl").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        thumb_url: json.get("thumbUrl").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
        client_file_id: json.get("clientFileId").and_then(|v| v.as_i64()).unwrap_or(0),
        chunk_id: json.get("chunkId").and_then(|v| v.as_i64()).unwrap_or(0),
    })
}
