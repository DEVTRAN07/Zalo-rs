//! Get stickers detail by IDs.

use futures_util::future::join_all;
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

pub type GetStickersDetailResponse = Vec<Value>;

/// Get stickers detail by sticker IDs.
pub async fn get_stickers_detail(
    ctx: &ContextSession,
    service_map: &Value,
    sticker_ids: &[i64],
) -> Result<GetStickersDetailResponse> {
    if sticker_ids.is_empty() {
        return Err(ZaloError::Api(ZaloApiError::new("Missing sticker id")));
    }
    let base_url = service_url(service_map, "sticker", "/api/message/sticker/sticker_detail")?;
    let base = {
        let utils = FactoryUtils::new(ctx);
        utils.make_url(&base_url, &[], true)?
    };

    let tasks = sticker_ids.iter().map(|sid| {
        let base = base.clone();
        let sid = *sid;
        async move {
            let utils = FactoryUtils::new(ctx);
            let params = serde_json::json!({ "sid": sid });
            let encrypted = utils
                .encode_aes(&params.to_string())
                .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt message")))?;
            let res = utils.request_get(&url_with_params(&base, &encrypted)).await?;
            utils.resolve::<Value>(res).await
        }
    });

    let results = join_all(tasks).await;
    let mut stickers = Vec::new();
    for r in results {
        if let Ok(v) = r {
            stickers.push(v);
        }
    }
    Ok(stickers)
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
