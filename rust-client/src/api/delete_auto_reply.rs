//! Delete an auto-reply rule.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteAutoReplyResponse {
    #[serde(default)]
    pub item: i64,
    #[serde(default)]
    pub version: i64,
}

/// Delete an auto-reply rule.
pub async fn delete_auto_reply(
    client: &ApiClient,
    id: i64,
) -> Result<DeleteAutoReplyResponse> {
    let url = client.service_url("auto_reply", "/api/autoreply/delete")?;
    let url = client.make_url(&url, &[], true)?;

    let params = serde_json::json!({
        "cliLang": client.ctx().language,
        "id": id,
    });

    client.post_encrypted(&url, &params).await
}
