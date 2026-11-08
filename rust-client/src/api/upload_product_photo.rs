use serde::{Deserialize, Serialize};
use serde_json::Value;

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

/// Upload a product photo.
///
/// **STUB**: Multipart/form-data upload is not yet implemented in the Rust client.
/// This function currently returns an error. Use the TypeScript client or implement
/// multipart support (reqwest `multipart` feature) to enable this endpoint.
pub async fn upload_product_photo(
    _client: &ApiClient,
    _payload: UploadProductPhotoPayload,
) -> Result<UploadProductPhotoResponse> {
    Err(ZaloError::Api(ZaloApiError::new(
        "multipart upload not yet implemented (upload_product_photo)",
    )))
}
