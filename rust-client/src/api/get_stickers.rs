//! Get stickers by keyword.

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

pub type GetStickersResponse = Vec<i64>;

/// Get stickers by keyword (returns sticker IDs).
pub async fn get_stickers(
    ctx: &ContextSession,
    service_map: &Value,
    keyword: &str,
) -> Result<GetStickersResponse> {
    if keyword.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing keyword")));
    }
    let base_url = service_url(service_map, "sticker", "/api/message/sticker")?;
    let utils = FactoryUtils::new(ctx);
    let base = utils.make_url(&base_url, &[], true)?;
    let url = format!("{}/suggest/stickers", base.trim_end_matches('/'));

    let params = serde_json::json!({
        "keyword": keyword,
        "gif": 1,
        "guggy": 0,
        "imei": ctx.imei,
    });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt message")))?;

    let res = utils.request_get(&url_with_params(&url, &encrypted)).await?;
    let suggestions: Value = utils.resolve(res).await?;

    let mut sticker_ids: Vec<i64> = Vec::new();
    if let Some(arr) = suggestions.get("sugg_sticker").and_then(|v| v.as_array()) {
        for s in arr {
            if let Some(id) = s.get("sticker_id").and_then(|v| v.as_i64()) {
                sticker_ids.push(id);
            } else if let Some(id) = s.get("stickerId").and_then(|v| v.as_i64()) {
                sticker_ids.push(id);
            }
        }
    }
    Ok(sticker_ids)
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
