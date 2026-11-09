//! Get alias list.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AliasItem {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub alias: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAliasListResponse {
    #[serde(default)]
    pub items: Vec<AliasItem>,
    #[serde(rename = "updateTime")]
    #[serde(default)]
    pub update_time: String,
}

/// Get alias list.
pub async fn get_alias_list(
    client: &ApiClient,
    count: Option<i32>,
    page: Option<i32>,
) -> Result<GetAliasListResponse> {
    let url = client.service_url("alias", "/api/alias/list")?;
    let params = serde_json::json!({
        "page": page.unwrap_or(1),
        "count": count.unwrap_or(100),
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
