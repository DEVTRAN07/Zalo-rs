use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetReminderResponsesResponse {
    #[serde(rename = "rejectMember", default)]
    pub reject_member: Vec<String>,
    #[serde(rename = "acceptMember", default)]
    pub accept_member: Vec<String>,
}

/// Get reminder responses.
pub async fn get_reminder_responses(
    client: &ApiClient,
    reminder_id: &str,
) -> Result<GetReminderResponsesResponse> {
    let base_url = client.service_url("group_board", "/api/board/topic/listResponseEvent")?;
    let params = serde_json::json!({ "eventId": reminder_id });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = url_with_params(&base_url, &encrypted);
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
