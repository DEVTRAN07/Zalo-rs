//! Delete a bank account.

use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::bank::BankAccount;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBankAccountPayload {
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteBankAccountResponse {
    #[serde(rename = "hasMore", default)]
    pub has_more: bool,
    #[serde(default)]
    pub total: i64,
    #[serde(rename = "myBanks", default)]
    pub my_banks: Vec<BankAccount>,
}

/// Delete a bank account.
pub async fn delete_bank_account(
    client: &ApiClient,
    payload: DeleteBankAccountPayload,
) -> Result<DeleteBankAccountResponse> {
    let url = client.service_url("zimsg", "/api/transfer/delete")?;
    let url = client.make_url(&url, &[], true)?;

    let params = serde_json::json!({
        "account_id": payload.account_id,
        "is_default": payload.is_default,
        "language": client.ctx().language,
    });

    client.post_encrypted(&url, &params).await
}
