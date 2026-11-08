//! Get avatar list.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarPhoto {
    #[serde(rename = "photoId")]
    pub photo_id: String,
    pub thumbnail: String,
    pub url: String,
    #[serde(rename = "bkUrl")]
    pub bk_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAvatarListResponse {
    #[serde(rename = "albumId")]
    #[serde(default)]
    pub album_id: String,
    #[serde(rename = "nextPhotoId")]
    #[serde(default)]
    pub next_photo_id: String,
    #[serde(default)]
    pub has_more: i32,
    #[serde(default)]
    pub photos: Vec<AvatarPhoto>,
}

/// Get avatar list.
pub async fn get_avatar_list(
    client: &ApiClient,
    count: Option<i32>,
    page: Option<i32>,
) -> Result<GetAvatarListResponse> {
    let url = client.service_url("profile", "/api/social/avatar-list")?;
    let params = serde_json::json!({
        "page": page.unwrap_or(1),
        "albumId": "0",
        "count": count.unwrap_or(50),
        "imei": client.ctx().imei,
    });

    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let full_url = if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(&encrypted))
    } else {
        format!("{}?params={}", url, urlencoding::encode(&encrypted))
    };
    let res = client.request_get(&full_url).await?;
    client.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
