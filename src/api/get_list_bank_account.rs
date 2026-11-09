//! Get list bank account.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetListBankAccountResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get list bank account.
pub async fn get_list_bank_account(client: &ApiClient) -> Result<GetListBankAccountResponse> {
    let url = client.service_url("zcloud", "/api/bank/account/list")?;
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
