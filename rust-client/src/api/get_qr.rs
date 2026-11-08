use std::collections::HashMap;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

pub type GetQrResponse = HashMap<String, String>;

/// Get QR code for users.
pub async fn get_qr(client: &ApiClient, user_id: &[String]) -> Result<GetQrResponse> {
    if user_id.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing user id")));
    }
    let url = client.service_url("friend", "/api/friend/mget-qr")?;
    let params = serde_json::json!({ "fids": user_id });
    client.post_encrypted(&url, &params).await
}
