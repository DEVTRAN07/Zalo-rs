//! Get sticker category detail.

use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

pub type GetStickerCategoryDetailResponse = Vec<Value>;

/// Get sticker category detail.
pub async fn get_sticker_category_detail(
    ctx: &ContextSession,
    service_map: &Value,
    cate_id: i64,
) -> Result<GetStickerCategoryDetailResponse> {
    let base_url = service_url(
        service_map,
        "sticker",
        "/api/message/sticker/category/sticker_detail",
    )?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({ "cid": cate_id });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_get(&url_with_params(&url, &encrypted)).await?;
    utils.resolve(res).await
}

fn url_with_params(url: &str, params: &str) -> String {
    if url.contains('?') {
        format!("{}&params={}", url, urlencoding::encode(params))
    } else {
        format!("{}?params={}", url, urlencoding::encode(params))
    }
}
