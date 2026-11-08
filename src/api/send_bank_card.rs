use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
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
    client: &ApiClient,
    payload: SendBankCardPayload,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<SendBankCardResponse> {
    let base_url = client.service_url("zimsg", "/api/transfer/card")?;
    let url = client.make_url(&base_url, &[], true)?;

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
    client.post_encrypted(&url, &params).await
}
