use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::r#enum::Gender;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProfileInfo {
    pub name: String,
    /// Date of birth in YYYY-MM-DD format.
    pub dob: String,
    /// Gender of the user.
    pub gender: Gender,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProfileBiz {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cate: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProfilePayload {
    pub profile: UpdateProfileInfo,
    #[serde(default)]
    pub biz: Option<UpdateProfileBiz>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProfileResponse(pub Value);

/// Change account profile information.
pub async fn update_profile(
    client: &ApiClient,
    payload: UpdateProfilePayload,
) -> Result<UpdateProfileResponse> {
    let profile_json = serde_json::json!({
        "name": payload.profile.name,
        "dob": payload.profile.dob,
        "gender": payload.profile.gender as i32,
    });
    let biz = payload.biz.unwrap_or_default();
    let biz_json = serde_json::json!({
        "desc": biz.description,
        "cate": biz.cate,
        "addr": biz.address,
        "website": biz.website,
        "email": biz.email,
    });

    let params = serde_json::json!({
        "profile": profile_json.to_string(),
        "biz": biz_json.to_string(),
        "language": client.ctx().language,
    });

    let base_url = client.service_url("profile", "/api/social/profile/update")?;
    let url = client.make_url(&base_url, &[], true)?;
    let val: Value = client.post_encrypted(&url, &params).await?;
    Ok(UpdateProfileResponse(val))
}
