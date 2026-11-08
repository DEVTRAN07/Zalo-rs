//! Change account avatar.
//!
//! Note: multipart file upload is not yet fully supported in the Rust HTTP
//! helpers. This endpoint is implemented as a stub that returns an error
//! explaining the limitation. Use the TS client or extend `http` with
//! multipart support to enable this.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangeAccountAvatarResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Change account avatar from a local file path.
///
/// Not yet implemented in rust-client (requires multipart upload).
pub async fn change_account_avatar(
    _client: &ApiClient,
    _avatar_path: &str,
) -> Result<ChangeAccountAvatarResponse> {
    Err(ZaloError::Api(ZaloApiError::new(
        "change_account_avatar requires multipart upload, which is not yet implemented in rust-client",
    )))
}
