//! Get biz account.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BizInfo {
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default)]
    pub cate: Option<Value>,
    #[serde(default)]
    pub addr: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingStartPage {
    #[serde(default)]
    pub enable_biz_label: i32,
    #[serde(default)]
    pub enable_cate: i32,
    #[serde(default)]
    pub enable_add: i32,
    #[serde(default)]
    pub cta_profile: i32,
    #[serde(default)]
    pub cta_catalog: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBizAccountResponse {
    #[serde(default)]
    pub biz: Option<BizInfo>,
    #[serde(default)]
    pub setting_start_page: Option<SettingStartPage>,
    #[serde(rename = "pkgId")]
    #[serde(default)]
    pub pkg_id: i64,
}

/// Get biz account.
pub async fn get_biz_account(client: &ApiClient, friend_id: &str) -> Result<GetBizAccountResponse> {
    let url = client.service_url("profile", "/api/social/friend/get-bizacc")?;
    let params = serde_json::json!({
        "fid": friend_id,
    });

    client.post_encrypted(&url, &params).await
}
