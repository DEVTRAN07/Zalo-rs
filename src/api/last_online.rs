use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LastOnlineSettings {
    #[serde(rename = "show_online_status", default)]
    pub show_online_status: bool,
    #[serde(rename = "display_last_seen", default)]
    pub display_last_seen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LastOnlineResponse {
    #[serde(default)]
    pub last_seen: i64,
    #[serde(default)]
    pub settings: Option<LastOnlineSettings>,
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// Get last online of a user.
pub async fn last_online(client: &ApiClient, uid: &str) -> Result<LastOnlineResponse> {
    let base_url = client.service_url("profile", "/api/social/profile/lastOnline")?;
    let url = client.make_url(&base_url, &[("nretry", "0".to_string())], true)?;
    let params = serde_json::json!({ "uid": uid, "conv_type": 1, "imei": client.ctx().imei });
    client.post_encrypted(&url, &params).await
}
