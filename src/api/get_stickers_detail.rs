use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

pub type GetStickersDetailResponse = Vec<Value>;

/// Get stickers detail.
pub async fn get_stickers_detail(
    client: &ApiClient,
    sticker_ids: &[i64],
) -> Result<GetStickersDetailResponse> {
    let base_url = client.service_url("sticker", "/api/message/sticker/sticker_detail")?;
    let params = serde_json::json!({
        "sid": serde_json::to_string(sticker_ids).unwrap_or_else(|_| "[]".to_string()),
    });
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = if base_url.contains('?') { format!("{}&params={}", &base_url, urlencoding::encode(&encrypted)) } else { format!("{}?params={}", &base_url, urlencoding::encode(&encrypted)) };
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
