//! Send bank card.

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::ThreadType;

pub type SendBankCardResponse = Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SendBankCardPayload {
    #[serde(rename = "binBank")]
    pub bin_bank: Value,
    #[serde(rename = "numAccBank")]
    pub num_acc_bank: String,
    #[serde(rename = "nameAccBank")]
    pub name_acc_bank: Option<String>,
}

/// Send bank card.
pub async fn send_bank_card(
    ctx: &ContextSession,
    service_map: &Value,
    payload: SendBankCardPayload,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<SendBankCardResponse> {
    let base_url = service_url(service_map, "zimsg", "/api/transfer/card")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let now = chrono::Utc::now().timestamp_millis();
    let params = serde_json::json!({
        "binBank": payload.bin_bank,
        "numAccBank": payload.num_acc_bank,
        "nameAccBank": payload.name_acc_bank.unwrap_or_else(|| "---".to_string()).to_uppercase(),
        "cliMsgId": now.to_string(),
        "tsMsg": now,
        "destUid": thread_id,
        "destType": if matches!(thread_type, ThreadType::Group) { 1 } else { 0 },
    });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
