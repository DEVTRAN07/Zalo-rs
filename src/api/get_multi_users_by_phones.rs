//! Get multi users by phones.

use crate::api::client::ApiClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMultiUsersByPhonesResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Get multi users by phones.
pub async fn get_multi_users_by_phones(
    client: &ApiClient,
    phones: Vec<String>,
) -> Result<GetMultiUsersByPhonesResponse> {
    let url = client.service_url("friend", "/api/friend/phonebook")?;
    let params = serde_json::json!({
        "phones": phones,
    });

    client.post_encrypted(&url, &params).await
}
