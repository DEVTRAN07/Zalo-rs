use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::r#enum::ThreadType;

/// Source for an attachment: either a local file path or in-memory data.
/// Full binary multipart upload is not yet implemented in the Rust client.
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

/// Upload attachment(s) to a thread.
///
/// **STUB**: Multipart/form-data chunked upload is not yet implemented in the Rust client.
/// This function currently returns an error. Use the TypeScript client or implement
/// multipart support (reqwest `multipart` feature) to enable this endpoint.
pub async fn upload_attachment(
    _client: &ApiClient,
    _sources: Vec<AttachmentSource>,
    _thread_id: &str,
    _thread_type: ThreadType,
) -> Result<UploadAttachmentResponse> {
    Err(ZaloError::Api(ZaloApiError::new(
        "multipart upload not yet implemented (upload_attachment)",
    )))
}
