use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum UpdateLangAvailableLanguages {
    #[default]
    VI,
    EN,
}

impl UpdateLangAvailableLanguages {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VI => "VI",
            Self::EN => "EN",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateLangResponse(pub Value);

/// Update language. Note: calling this alone may not fully update the user's language.
pub async fn update_lang(
    client: &ApiClient,
    language: Option<UpdateLangAvailableLanguages>,
) -> Result<UpdateLangResponse> {
    let lang = language.unwrap_or_default();
    let params = serde_json::json!({ "language": lang.as_str() });

    let base_url = client.service_url("profile", "/api/social/profile/updatelang")?;
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    let val: Value = client.resolve(res).await?;
    Ok(UpdateLangResponse(val))
}
