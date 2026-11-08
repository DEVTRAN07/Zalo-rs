//! Get group invite box list.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupInviteBoxListResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get group invite box list.
pub async fn get_group_invite_box_list(client: &ApiClient) -> Result<GetGroupInviteBoxListResponse> {
    let url = client.service_url("group", "/api/group/invite-box/list")?;
    let params = serde_json::json!({});

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
