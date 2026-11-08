//! Delete avatar(s) from the avatar list.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AvatarDeleteError {
    #[serde(default)]
    pub err: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteAvatarResponse {
    #[serde(rename = "delPhotoIds", default)]
    pub del_photo_ids: Vec<String>,
    #[serde(rename = "errMap", default)]
    pub err_map: HashMap<String, AvatarDeleteError>,
}

/// Delete avatar photo(s) by ID.
pub async fn delete_avatar(
    client: &ApiClient,
    photo_ids: Vec<String>,
) -> Result<DeleteAvatarResponse> {
    let url = client.service_url("profile", "/api/social/del-avatars")?;

    let del_photos: Vec<serde_json::Value> = photo_ids
        .into_iter()
        .map(|id| serde_json::json!({ "photoId": id }))
        .collect();

    let params = serde_json::json!({
        "delPhotos": serde_json::to_string(&del_photos).unwrap_or_else(|_| "[]".to_string()),
        "imei": client.ctx().imei,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = client.make_url(&url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
