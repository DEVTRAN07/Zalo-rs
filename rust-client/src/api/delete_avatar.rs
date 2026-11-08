//! Delete avatar(s) from the avatar list.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

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
    ctx: &ContextSession,
    service_map: &serde_json::Value,
    photo_ids: Vec<String>,
) -> Result<DeleteAvatarResponse> {
    let base_url = service_url(service_map, "profile", "/api/social/del-avatars")?;
    let utils = FactoryUtils::new(ctx);

    let del_photos: Vec<serde_json::Value> = photo_ids
        .into_iter()
        .map(|id| serde_json::json!({ "photoId": id }))
        .collect();

    let params = serde_json::json!({
        "delPhotos": serde_json::to_string(&del_photos).unwrap_or_else(|_| "[]".to_string()),
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = utils.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
