//! Get group invite box info.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupInviteBoxInfoResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get group invite box info.
pub async fn get_group_invite_box_info(
    client: &ApiClient,
    group_id: &str,
) -> Result<GetGroupInviteBoxInfoResponse> {
    let url = client.service_url("group", "/api/group/invite-box/info")?;
    let params = serde_json::json!({
        "grid": group_id,
    });

    client.post_encrypted(&url, &params).await
}
