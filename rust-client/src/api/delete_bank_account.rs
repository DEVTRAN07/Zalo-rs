//! Delete a bank account.

use serde::{Deserialize, Serialize};

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
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
    ctx: &ContextSession,
    service_map: &serde_json::Value,
    payload: DeleteBankAccountPayload,
) -> Result<DeleteBankAccountResponse> {
    let base_url = service_url(service_map, "zimsg", "/api/transfer/delete")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "account_id": payload.account_id,
        "is_default": payload.is_default,
        "language": ctx.language,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
