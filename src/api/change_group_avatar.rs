//! Change group avatar.
//!
//! Note: multipart file upload is not yet fully supported in the Rust HTTP
//! helpers. This endpoint is implemented as a stub.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangeGroupAvatarResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Change group avatar from a local file path.
///
/// Not yet implemented in rust-client (requires multipart upload).
pub async fn change_group_avatar(
    _client: &ApiClient,
    _avatar_path: &str,
    _group_id: &str,
) -> Result<ChangeGroupAvatarResponse> {
    Err(ZaloError::Api(ZaloApiError::new(
        "change_group_avatar requires multipart upload, which is not yet implemented in rust-client",
    )))
}
