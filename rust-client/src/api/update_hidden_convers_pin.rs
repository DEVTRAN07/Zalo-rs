use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateHiddenConversPinResponse(pub serde_json::Value);

/// Update hidden conversation PIN (must be a 4-digit number 0000-9999).
pub async fn update_hidden_convers_pin(
    client: &ApiClient,
    pin: &str,
) -> Result<UpdateHiddenConversPinResponse> {
    if pin.len() != 4 || !pin.chars().all(|c| c.is_ascii_digit()) {
        return Err(ZaloError::Api(ZaloApiError::new(
            "Pin must be a 4-digit number between 0000-9999",
        )));
    }

    let encrypted_pin = crate::crypto::encrypt_pin(pin);
    let params = serde_json::json!({
        "new_pin": encrypted_pin,
        "imei": client.ctx().imei,
    });

    let base_url = client.service_url("conversation", "/api/hiddenconvers/update-pin")?;
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    let val: serde_json::Value = client.resolve(res).await?;
    Ok(UpdateHiddenConversPinResponse(val))
}
