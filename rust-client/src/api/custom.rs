//! Custom API extension point.
//!
//! The TypeScript client allows injecting custom endpoints onto the API object
//! at runtime. Rust is statically typed, so this module provides a small
//! helper type for building one-off encrypted requests using the same
//! session/context plumbing.

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;

/// Helper for making custom Zalo API calls with the standard encrypt/resolve flow.
pub struct CustomApi<'a> {
    ctx: &'a ContextSession,
    service_map: &'a Value,
}

impl<'a> CustomApi<'a> {
    pub fn new(ctx: &'a ContextSession, service_map: &'a Value) -> Self {
        Self { ctx, service_map }
    }

    pub fn service_map(&self) -> &Value {
        self.service_map
    }

    pub fn ctx(&self) -> &ContextSession {
        self.ctx
    }

    /// Make an encrypted POST request to a fully constructed URL.
    pub async fn post_encrypted<T: DeserializeOwned>(
        &self,
        url: &str,
        params: &Value,
    ) -> Result<T> {
        let utils = FactoryUtils::new(self.ctx);
        let encrypted = utils
            .encode_aes(&params.to_string())
            .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
        let res = utils.request_post(url, &[("params", encrypted)]).await?;
        utils.resolve(res).await
    }

    /// Make an encrypted GET request with `params` in the query string.
    pub async fn get_encrypted<T: DeserializeOwned>(
        &self,
        base_url: &str,
        params: &Value,
    ) -> Result<T> {
        let utils = FactoryUtils::new(self.ctx);
        let encrypted = utils
            .encode_aes(&params.to_string())
            .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
        let url = utils.make_url(base_url, &[("params", encrypted)], true)?;
        let res = utils.request_get(&url).await?;
        utils.resolve(res).await
    }
}
