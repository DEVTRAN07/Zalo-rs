//! Get related friend group.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRelatedFriendGroupResponse {
    #[serde(rename = "groupRelateds", default)]
    pub group_relateds: HashMap<String, Vec<String>>,
}

/// Get related friend group (zBusiness).
pub async fn get_related_friend_group(
    ctx: &ContextSession,
    service_map: &Value,
    friend_ids: &[String],
) -> Result<GetRelatedFriendGroupResponse> {
    let base_url = service_url(service_map, "friend", "/api/friend/group/related")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "friend_ids": serde_json::to_string(friend_ids).unwrap_or_else(|_| "[]".to_string()),
        "imei": ctx.imei,
    });
    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
