//! Create a bank account.

use serde::{Deserialize, Serialize};

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::core::utils::normalize_holder_name;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
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
    ctx: &ContextSession,
    service_map: &serde_json::Value,
    payload: CreateBankAccountPayload,
) -> Result<CreateBankAccountResponse> {
    let base_url = service_url(service_map, "zimsg", "/api/transfer/create")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let holder = normalize_holder_name(Some(&payload.name_acc_bank))
        .unwrap_or_else(|| payload.name_acc_bank.clone());
    let params = serde_json::json!({
        "bin": payload.bin_bank,
        "bank_number": payload.num_acc_bank,
        "holder_name": holder,
        "language": ctx.language,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
