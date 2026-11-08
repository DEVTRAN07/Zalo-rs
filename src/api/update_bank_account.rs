use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::core::utils::normalize_holder_name;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::bank::{BankAccount, BinBankCard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBankAccountPayload {
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(rename = "binBank")]
    pub bin_bank: BinBankCard,
    #[serde(rename = "numAccBank")]
    pub num_acc_bank: String,
    #[serde(rename = "nameAccBank")]
    pub name_acc_bank: String,
}

pub type UpdateBankAccountResponse = BankAccount;

/// Update bank account.
pub async fn update_bank_account(
    client: &ApiClient,
    payload: UpdateBankAccountPayload,
) -> Result<UpdateBankAccountResponse> {
    let holder = normalize_holder_name(Some(&payload.name_acc_bank))
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Missing bank account holder name")))?;
    let params = serde_json::json!({
        "account_id": payload.account_id,
        "bin": payload.bin_bank as i64,
        "bank_number": payload.num_acc_bank,
        "holder_name": holder,
        "language": client.ctx().language,
    });

    let url = client.service_url("zimsg", "/api/transfer/update")?;
    client.post_encrypted(&url, &params).await
}
