use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

pub type ReuseAvatarResponse = Value;

/// Reuse avatar.
pub async fn reuse_avatar(
    client: &ApiClient,
    photo_id: &str,
    is_hd: Option<bool>,
) -> Result<ReuseAvatarResponse> {
    let base_url = client.service_url("profile", "/api/social/reuse-avatar")?;
    let params = serde_json::json!({
        "photoId": photo_id,
        "isHD": is_hd.unwrap_or(false),
        "imei": client.ctx().imei,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = if base_url.contains('?') { format!("{}&params={}", &base_url, urlencoding::encode(&encrypted)) } else { format!("{}?params={}", &base_url, urlencoding::encode(&encrypted)) };
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
