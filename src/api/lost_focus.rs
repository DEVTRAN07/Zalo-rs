use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type LostFocusResponse = Value;

/// Lost focus.
pub async fn lost_focus(client: &ApiClient) -> Result<LostFocusResponse> {
    let base_url = client.service_url("presence", "/api/presence/lost-focus")?;
    let url = client.make_url(&base_url, &[], true)?;
    let params = serde_json::json!({});
    client.post_encrypted(&url, &params).await
}
