use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

pub type GetStickerCategoryDetailResponse = Value;

/// Get sticker category detail.
pub async fn get_sticker_category_detail(
    client: &ApiClient,
    cid: i64,
) -> Result<GetStickerCategoryDetailResponse> {
    let base_url = client.service_url("sticker", "/api/message/sticker/cate-detail")?;
    let params = serde_json::json!({ "cid": cid, "imei": client.ctx().imei });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = if base_url.contains('?') { format!("{}&params={}", &base_url, urlencoding::encode(&encrypted)) } else { format!("{}?params={}", &base_url, urlencoding::encode(&encrypted)) };
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
