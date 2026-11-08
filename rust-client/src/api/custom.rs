//! Custom API extension point.
//!
//! The TypeScript client allows injecting custom endpoints onto the API object
//! at runtime. Rust is statically typed, so this module provides a small
//! helper type for building one-off encrypted requests using the same
//! session/context plumbing.

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};

/// Helper for making custom Zalo API calls with the standard encrypt/resolve flow.
pub struct CustomApi {
    client: ApiClient,
}

impl CustomApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub fn service_map(&self) -> &Value {
        self.client.service_map()
    }

    pub fn ctx(&self) -> &ContextSession {
        self.client.ctx()
    }

    /// Make an encrypted POST request to a fully constructed URL.
    pub async fn post_encrypted<T: DeserializeOwned>(
        &self,
        url: &str,
        params: &Value,
    ) -> Result<T> {
        self.client.post_encrypted(url, params).await
    }

    /// Make an encrypted GET request with `params` in the query string.
    pub async fn get_encrypted<T: DeserializeOwned>(
        &self,
        base_url: &str,
        params: &Value,
    ) -> Result<T> {
        let encrypted = self
            .client
            .encode_aes(&params.to_string())
            .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
        let url = self.client.make_url(base_url, &[("params", encrypted)], true)?;
        let res = self.client.request_get(&url).await?;
        self.client.resolve(res).await
    }
}
