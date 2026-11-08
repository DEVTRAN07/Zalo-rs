//! Get biz account.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

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
pub async fn get_biz_account(ctx: &ContextSession, service_map: &Value, friend_id: &str) -> Result<GetBizAccountResponse> {
    let base_url = service_url(service_map, "profile", "/api/social/friend/get-bizacc")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "fid": friend_id,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils
        .request_post(&url, &[("params", encrypted)])
        .await?;
    utils.resolve(res).await
}
