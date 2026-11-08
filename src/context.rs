//! Session context: configuration, credentials, and runtime state.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Retry / timeout policy for outgoing HTTP requests.
#[derive(Debug, Clone)]
pub struct RetryOptions {
    /// Maximum retry attempts after the initial request. Default `3`.
    pub limit: u32,
    /// Base delay (ms) of the exponential backoff. Default `250`.
    pub delay_ms: u64,
    /// Upper bound (ms) for a single backoff delay. Default `5000`.
    pub max_delay_ms: u64,
    /// HTTP methods eligible for retry. Default `["GET", "HEAD"]`.
    pub methods: Vec<String>,
    /// Overall request timeout (ms). Default `60000`. Use `0` to disable.
    pub timeout_ms: u64,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            limit: 3,
            delay_ms: 250,
            max_delay_ms: 5000,
            methods: vec!["GET".into(), "HEAD".into()],
            timeout_ms: 60_000,
        }
    }
}

/// Log level setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Logging {
    Silent,
    Error,
    Warn,
    Info,
    Verbose,
}

/// Client options.
#[derive(Debug, Clone)]
pub struct Options {
    /// Receive events triggered by the current account (self listen).
    pub self_listen: bool,
    /// Logging level.
    pub logging: Logging,
    pub api_type: u32,
    pub api_version: u32,
    pub retry: RetryOptions,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            self_listen: false,
            logging: Logging::Info,
            api_type: 30,
            api_version: 685,
            retry: RetryOptions::default(),
        }
    }
}

/// A browser cookie in the JSON-array format used by Zalo credential import.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZaloCookie {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub expiration_date: Option<f64>,
    #[serde(default)]
    pub host_only: Option<bool>,
    #[serde(default)]
    pub http_only: Option<bool>,
    #[serde(default)]
    pub secure: Option<bool>,
    #[serde(default)]
    pub same_site: Option<String>,
    #[serde(default)]
    pub session: Option<bool>,
    #[serde(default)]
    pub store_id: Option<String>,
    /// Support `key` field from tough-cookie serialized format
    #[serde(default)]
    pub key: Option<String>,
}

/// Minimal credential shape used by Zalo login flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZaloCredentials {
    pub imei: String,
    pub cookie: Vec<ZaloCookie>,
    pub user_agent: String,
    #[serde(default)]
    pub language: Option<String>,
}

/// Alias used by examples / public API.
pub type Credentials = ZaloCredentials;

/// Login info returned by the getLoginInfo API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginInfo {
    #[serde(default)]
    pub uid: Option<String>,
    #[serde(default)]
    pub zpw_enk: Option<String>,
    #[serde(default)]
    pub zpw_ws: Option<Vec<String>>,
    #[serde(default)]
    pub zpw_service_map_v3: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Service map (zpw_service_map_v3).
pub type ZpwServiceMap = serde_json::Value;

/// Server info settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerInfo {
    #[serde(default)]
    pub settings: Option<serde_json::Value>,
    #[serde(default)]
    pub setttings: Option<serde_json::Value>, // Zalo typo fallback
    #[serde(default)]
    pub extra_ver: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Shared HTTP client state (cookie jar + reqwest client).
#[derive(Clone)]
pub struct HttpState {
    pub client: reqwest::Client,
    pub cookie_store: Arc<reqwest_cookie_store::CookieStoreMutex>,
}

impl std::fmt::Debug for HttpState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpState").finish_non_exhaustive()
    }
}

/// Base context for pre-login operations.
#[derive(Clone)]
pub struct ContextBase {
    pub api_type: u32,
    pub api_version: u32,
    pub imei: Option<String>,
    pub user_agent: Option<String>,
    pub language: Option<String>,
    pub secret_key: Option<String>,
    pub uid: Option<String>,
    pub options: Options,
    pub http: Arc<Mutex<Option<HttpState>>>,
    pub login_info: Option<LoginInfo>,
    pub settings: Option<serde_json::Value>,
    pub extra_ver: Option<serde_json::Value>,
}

impl std::fmt::Debug for ContextBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextBase")
            .field("api_type", &self.api_type)
            .field("api_version", &self.api_version)
            .field("imei", &self.imei)
            .field("uid", &self.uid)
            .finish_non_exhaustive()
    }
}

/// Authenticated session context.
#[derive(Debug, Clone)]
pub struct ContextSession {
    pub api_type: u32,
    pub api_version: u32,
    pub imei: String,
    pub user_agent: String,
    pub language: String,
    pub secret_key: String,
    pub uid: String,
    pub options: Options,
    pub http: Arc<Mutex<Option<HttpState>>>,
    pub login_info: LoginInfo,
    pub settings: Option<serde_json::Value>,
    pub extra_ver: Option<serde_json::Value>,
}

impl ContextSession {
    pub fn from_base(base: &ContextBase) -> Option<Self> {
        Some(Self {
            api_type: base.api_type,
            api_version: base.api_version,
            imei: base.imei.clone()?,
            user_agent: base.user_agent.clone()?,
            language: base.language.clone().unwrap_or_else(|| "vi".into()),
            secret_key: base.secret_key.clone()?,
            uid: base.uid.clone()?,
            options: base.options.clone(),
            http: base.http.clone(),
            login_info: base.login_info.clone().unwrap_or_default(),
            settings: base.settings.clone(),
            extra_ver: base.extra_ver.clone(),
        })
    }
}

/// Create a default pre-login context.
pub fn create_context(api_type: u32, api_version: u32) -> ContextBase {
    ContextBase {
        api_type,
        api_version,
        imei: None,
        user_agent: None,
        language: None,
        secret_key: None,
        uid: None,
        options: Options {
            api_type,
            api_version,
            ..Options::default()
        },
        http: Arc::new(Mutex::new(None)),
        login_info: None,
        settings: None,
        extra_ver: None,
    }
}

pub fn is_context_session(ctx: &ContextBase) -> bool {
    ctx.secret_key.is_some() && ctx.uid.is_some() && ctx.imei.is_some()
}
