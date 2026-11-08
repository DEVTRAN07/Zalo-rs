//! Get auto delete chat.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoDeleteConvers {
    #[serde(rename = "destId")]
    pub dest_id: String,
    #[serde(rename = "isGroup")]
    pub is_group: bool,
    pub ttl: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAutoDeleteChatResponse {
    #[serde(default)]
    pub convers: Vec<AutoDeleteConvers>,
}

/// Get auto delete chat.
pub async fn get_auto_delete_chat(ctx: &ContextSession, service_map: &Value) -> Result<GetAutoDeleteChatResponse> {
    let base_url = service_url(service_map, "conversation", "/api/conv/autodelete/getConvers")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({});

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
