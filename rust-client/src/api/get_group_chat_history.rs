//! Get group chat history.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupChatHistoryResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get group chat history.
pub async fn get_group_chat_history(
    ctx: &ContextSession,
    service_map: &Value,
    group_id: &str,
    last_id: Option<i64>,
    count: Option<i32>,
) -> Result<GetGroupChatHistoryResponse> {
    let base_url = service_url(service_map, "group", "/api/group/getmsgv2")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "grid": group_id,
        "lastId": last_id.unwrap_or(0),
        "count": count.unwrap_or(50),
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
