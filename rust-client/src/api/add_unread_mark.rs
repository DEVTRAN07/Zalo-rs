//! Add unread mark to a conversation.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddUnreadMarkResponse {
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub status: i64,
}

/// Add unread mark to a conversation.
pub async fn add_unread_mark(
    ctx: &ContextSession,
    service_map: &Value,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<AddUnreadMarkResponse> {
    let base_url = service_url(service_map, "conversation", "/api/conv/addUnreadMark")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;
    let ts_str = ts.to_string();
    let is_group = matches!(thread_type, ThreadType::Group);
    let key = if is_group { "convsGroup" } else { "convsUser" };
    let other = if is_group { "convsUser" } else { "convsGroup" };

    let param = serde_json::json!({
        key: [{
            "id": thread_id,
            "cliMsgId": ts_str,
            "fromUid": "0",
            "ts": ts,
        }],
        other: [],
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&param.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils
        .request_post(&url, &[("params", encrypted)])
        .await?;
    utils.resolve(res).await
}
