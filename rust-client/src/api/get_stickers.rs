use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

pub type GetStickersResponse = Vec<i64>;

/// Get stickers by keyword (returns sticker IDs).
pub async fn get_stickers(
    client: &ApiClient,
    keyword: &str,
) -> Result<GetStickersResponse> {
    if keyword.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing keyword")));
    }
    let base = client.service_url("sticker", "/api/message/sticker")?;
    let url_base = format!("{}/suggest/stickers", base.trim_end_matches('/'));

    let params = serde_json::json!({
        "keyword": keyword,
        "gif": 1,
        "guggy": 0,
        "imei": client.ctx().imei,
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt message")))?;

    let url = url_with_params(&url_base, &encrypted);
    let res = client.request_get(&url).await?;
    let suggestions: Value = client.resolve(res).await?;

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
