//! Get friend onlines.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFriendOnlinesStatus {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetFriendOnlinesResponse {
    #[serde(default)]
    pub predefine: Vec<String>,
    #[serde(rename = "ownerStatus")]
    #[serde(default)]
    pub owner_status: String,
    #[serde(default)]
    pub onlines: Vec<GetFriendOnlinesStatus>,
}

/// Get friend onlines.
pub async fn get_friend_onlines(ctx: &ContextSession, service_map: &Value) -> Result<GetFriendOnlinesResponse> {
    let base_url = service_url(service_map, "profile", "/api/social/friend/onlines")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_get(&url_with_params(&url, &encrypted)).await?;
    let mut result: GetFriendOnlinesResponse = utils.resolve(res).await?;
    for online in &mut result.onlines {
        if let Ok(parsed) = serde_json::from_str::<Value>(&online.status) {
            if let Some(status) = parsed.get("status").and_then(|v| v.as_str()) {
                online.status = status.to_string();
            }
        }
    }
    Ok(result)
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
