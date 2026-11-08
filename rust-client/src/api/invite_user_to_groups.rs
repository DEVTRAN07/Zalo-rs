//! Invite user to groups.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InviteUserToGroupsResponse {
    #[serde(rename = "grid_message_map", default, flatten)]
    pub data: Value,
}

/// Invite user to group(s).
pub async fn invite_user_to_groups(
    ctx: &ContextSession,
    service_map: &Value,
    user_id: &str,
    group_ids: &[String],
) -> Result<InviteUserToGroupsResponse> {
    let base_url = service_url(service_map, "group", "/api/group/invite/multi")?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let params = serde_json::json!({
        "grids": group_ids,
        "member": user_id,
        "memberType": -1,
        "srcInteraction": 2,
        "clientLang": ctx.language,
    });
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
