use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetReminderResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get reminder details from a group.
pub async fn get_reminder(
    client: &ApiClient,
    reminder_id: &str,
) -> Result<GetReminderResponse> {
    let base_url = client.service_url("group_board", "/api/board/topic/getReminder")?;
    let params = serde_json::json!({
        "eventId": reminder_id,
        "imei": client.ctx().imei,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = if base_url.contains('?') { format!("{}&params={}", &base_url, urlencoding::encode(&encrypted)) } else { format!("{}?params={}", &base_url, urlencoding::encode(&encrypted)) };
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
