//! Get multi users by phones.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMultiUsersByPhonesResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get multi users by phones.
pub async fn get_multi_users_by_phones(
    ctx: &ContextSession,
    service_map: &Value,
    phones: Vec<String>,
) -> Result<GetMultiUsersByPhonesResponse> {
    let base_url = service_url(service_map, "friend", "/api/friend/phonebook")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "phones": phones,
    });

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
