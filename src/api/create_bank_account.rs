//! Create a bank account.

use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::core::utils::normalize_holder_name;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::models::bank::BankAccount;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBankAccountPayload {
    /// BIN code of the bank card.
    #[serde(rename = "binBank")]
    pub bin_bank: i64,
    #[serde(rename = "numAccBank")]
    pub num_acc_bank: String,
    #[serde(rename = "nameAccBank")]
    pub name_acc_bank: String,
}

pub type CreateBankAccountResponse = BankAccount;

/// Create a bank account.
pub async fn create_bank_account(
    client: &ApiClient,
    payload: CreateBankAccountPayload,
) -> Result<CreateBankAccountResponse> {
    let url = client.service_url("zimsg", "/api/transfer/create")?;
    let url = client.make_url(&url, &[], true)?;

    let holder = normalize_holder_name(Some(&payload.name_acc_bank))
        .unwrap_or_else(|| payload.name_acc_bank.clone());
    let params = serde_json::json!({
        "bin": payload.bin_bank,
        "bank_number": payload.num_acc_bank,
        "holder_name": holder,
        "language": client.ctx().language,
    });

    client.post_encrypted(&url, &params).await
}
