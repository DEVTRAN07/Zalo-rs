use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

pub type JoinGroupLinkResponse = Value;

/// Join group via invite link.
pub async fn join_group_link(client: &ApiClient, link: &str) -> Result<JoinGroupLinkResponse> {
    let base_url = client.service_url("group", "/api/group/link/join")?;
    let params = serde_json::json!({
        "link": link,
        "clientLang": client.ctx().language,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = url_with_params(&base_url, &encrypted);
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
