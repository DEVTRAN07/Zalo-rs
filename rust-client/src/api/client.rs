//! `ApiClient` — idiomatic request context for Zalo API calls.

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::context::{ContextBase, ContextSession};
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::http;

use super::url;

pub type ZpwServiceMap = Value;

#[derive(Debug, Clone)]
pub struct ApiClient {
    ctx: ContextSession,
    service_map: ZpwServiceMap,
}

impl ApiClient {
    pub fn new(ctx: ContextSession, service_map: ZpwServiceMap) -> Self {
        Self { ctx, service_map }
    }

    pub fn ctx(&self) -> &ContextSession {
        &self.ctx
    }

    pub fn service_map(&self) -> &ZpwServiceMap {
        &self.service_map
    }

    /// Resolve a service URL from zpw_service_map.
    pub fn service_url(&self, service: &str, path: &str) -> Result<String> {
        url::service_url(&self.service_map, service, path)
    }

    /// Build a URL with query params and API version/type markers.
    pub fn make_url(
        &self,
        base_url: &str,
        params: &[(&str, String)],
        api_version: bool,
    ) -> Result<String> {
        let base = self.ctx_to_base();
        http::make_url(&base, base_url, params, api_version)
    }

    /// Encrypt data with session secret key (base64).
    pub fn encode_aes(&self, data: &str) -> Option<String> {
        crate::crypto::encode_aes(&self.ctx.secret_key, data)
    }

    /// POST form data to a URL.
    pub async fn request_post(&self, url: &str, form: &[(&str, String)]) -> Result<reqwest::Response> {
        http::post_form(&self.ctx_to_base(), url, form).await
    }

    /// GET a URL.
    pub async fn request_get(&self, url: &str) -> Result<reqwest::Response> {
        http::get(&self.ctx_to_base(), url).await
    }

    /// Resolve an encrypted response into typed data.
    pub async fn resolve<T: DeserializeOwned>(&self, res: reqwest::Response) -> Result<T> {
        crate::factory::resolve_response(&self.ctx, res, true).await
    }

    /// Resolve an unencrypted response into typed data.
    pub async fn resolve_plain<T: DeserializeOwned>(&self, res: reqwest::Response) -> Result<T> {
        crate::factory::resolve_response(&self.ctx, res, false).await
    }

    /// Convenience: encrypted POST with JSON params body.
    pub async fn post_encrypted<T: DeserializeOwned>(&self, url: &str, params: &Value) -> Result<T> {
        let encrypted = self
            .encode_aes(&params.to_string())
            .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
        let res = self.request_post(url, &[("params", encrypted)]).await?;
        self.resolve(res).await
    }

    fn ctx_to_base(&self) -> ContextBase {
        ContextBase {
            api_type: self.ctx.api_type,
            api_version: self.ctx.api_version,
            imei: Some(self.ctx.imei.clone()),
            user_agent: Some(self.ctx.user_agent.clone()),
            language: Some(self.ctx.language.clone()),
            secret_key: Some(self.ctx.secret_key.clone()),
            uid: Some(self.ctx.uid.clone()),
            options: self.ctx.options.clone(),
            http: self.ctx.http.clone(),
            login_info: Some(self.ctx.login_info.clone()),
            settings: self.ctx.settings.clone(),
            extra_ver: self.ctx.extra_ver.clone(),
        }
    }
}
