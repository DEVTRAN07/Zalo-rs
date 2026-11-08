//! Get avatar list.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

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
    ctx: &ContextSession,
    service_map: &Value,
    count: Option<i32>,
    page: Option<i32>,
) -> Result<GetAvatarListResponse> {
    let base_url = service_url(service_map, "profile", "/api/social/avatar-list")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "page": page.unwrap_or(1),
        "albumId": "0",
        "count": count.unwrap_or(50),
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_get(&url_with_params(&url, &encrypted)).await?;
    utils.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
