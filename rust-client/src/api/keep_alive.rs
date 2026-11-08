//! Keep-alive endpoint.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::Result;
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeepAliveResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Keep the session alive.
///
/// Uses the chat keepalive endpoint. The exact payload is minimal; failures
/// are non-fatal in typical usage.
pub async fn keep_alive(ctx: &ContextSession, service_map: &Value) -> Result<KeepAliveResponse> {
    // Zalo's keepAlive varies; use chat service as a common target.
    let base_url = service_url(service_map, "chat", "/keepalive").unwrap_or_else(|_| {
        "https://wpa.chat.zalo.me/api/chat/keepalive".to_string()
    });
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "imei": ctx.imei,
    });
    if let Some(encrypted) = utils.encode_aes(&params.to_string()) {
        let res = utils.request_post(&url, &[("params", encrypted)]).await?;
        // Try to resolve; if server returns unexpected format, return default.
        match utils.resolve::<KeepAliveResponse>(res).await {
            Ok(v) => Ok(v),
            Err(_) => Ok(KeepAliveResponse::default()),
        }
    } else {
        Ok(KeepAliveResponse::default())
    }
}
