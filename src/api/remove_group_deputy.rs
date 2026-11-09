use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

pub type RemoveGroupDeputyResponse = Value;

/// Remove group deputy.
pub async fn remove_group_deputy(
    client: &ApiClient,
    member_id: &str,
    group_id: &str,
) -> Result<RemoveGroupDeputyResponse> {
    let base_url = client.service_url("group", "/api/group/admins/remove")?;
    let params = serde_json::json!({
        "grid": group_id,
        "members": [member_id],
        "imei": client.ctx().imei,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = if base_url.contains('?') { format!("{}&params={}", &base_url, urlencoding::encode(&encrypted)) } else { format!("{}?params={}", &base_url, urlencoding::encode(&encrypted)) };
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
