//! Change a group's name.

use serde::{Deserialize, Serialize};

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangeGroupNameResponse {
    #[serde(default)]
    pub status: i64,
}

/// Change group name.
pub async fn change_group_name(
    client: &ApiClient,
    name: &str,
    group_id: &str,
) -> Result<ChangeGroupNameResponse> {
    let url = client.service_url("group", "/api/group/updateinfo")?;
    let url = client.make_url(&url, &[], true)?;

    let gname = if name.is_empty() {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .to_string()
    } else {
        name.to_string()
    };

    let params = serde_json::json!({
        "grid": group_id,
        "gname": gname,
        "imei": client.ctx().imei,
    });

    client.post_encrypted(&url, &params).await
}
