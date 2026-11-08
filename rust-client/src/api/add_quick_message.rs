//! Add a quick message.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::quick_message::QuickMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddQuickMessagePayload {
    pub keyword: String,
    pub title: String,
    /// Optional media; uploading via API is not yet wired in Rust.
    /// Pass a prebuilt media structure via `media_payload` if needed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddQuickMessageResponse {
    pub item: Option<QuickMessage>,
    #[serde(default)]
    pub version: i64,
}

/// Add a quick message.
///
/// Note: media upload (`uploadProductPhoto`) is not yet implemented in the
/// Rust crate. If `payload.media` is provided, this returns an error.
pub async fn add_quick_message(
    ctx: &ContextSession,
    service_map: &Value,
    payload: AddQuickMessagePayload,
) -> Result<AddQuickMessageResponse> {
    if payload.media.is_some() {
        return Err(ZaloError::Api(ZaloApiError::new(
            "Media upload for quick messages is not yet implemented in rust-client; set media to None",
        )));
    }

    let base_url = service_url(service_map, "quick_message", "/api/quickmessage/create")?;
    let utils = FactoryUtils::new(ctx);

    let params = serde_json::json!({
        "keyword": payload.keyword,
        "message": {
            "title": payload.title,
            "params": "",
        },
        "type": 0,
        "imei": ctx.imei,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let url = utils.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = utils.request_get(&url).await?;
    utils.resolve(res).await
}
