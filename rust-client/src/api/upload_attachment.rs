use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use tokio::fs;

use crate::api::client::ApiClient;
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
pub struct UploadAttachmentResponse(pub Vec<Value>);

pub async fn upload_attachment(
    client: &ApiClient,
    sources: Vec<AttachmentSource>,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<UploadAttachmentResponse> {
    let url = client.service_url("file", "/api/message/uploadAttachment")?;

    let mut form = reqwest::multipart::Form::new()
        .text("threadId", thread_id.to_string())
        .text("threadType", (thread_type as u8).to_string())
        .text("imei", client.ctx().imei.clone());

    for (idx, source) in sources.iter().enumerate() {
        match source {
            AttachmentSource::Path(path) => {
                let data = fs::read(path).await.map_err(|e| {
                    ZaloError::Api(ZaloApiError::new(format!("Failed to read file {}: {}", path, e)))
                })?;
                let filename = Path::new(path).file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("file")
                    .to_string();
                form = form.part(format!("file{}", idx),
                    reqwest::multipart::Part::bytes(data)
                        .file_name(filename)
                        .mime_str("application/octet-stream").unwrap());
            }
            AttachmentSource::Inline { filename, data, .. } => {
                form = form.part(format!("file{}", idx),
                    reqwest::multipart::Part::bytes(data.clone())
                        .file_name(filename.clone())
                        .mime_str("application/octet-stream").unwrap());
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

    let resp = client_req.headers(headers).send().await
        .map_err(ZaloError::Http)?;

    if !resp.status().is_success() {
        return Err(ZaloError::Api(ZaloApiError::new(format!(
            "Upload failed with status {}", resp.status()
        ))));
    }

    let json: Value = resp.json().await.map_err(ZaloError::Http)?;
    Ok(UploadAttachmentResponse(json.as_array().cloned().unwrap_or_default()))
}
