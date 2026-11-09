//! High-level client: authenticates and returns a ready-to-use API instance.

use reqwest_cookie_store::CookieStoreMutex;
use std::sync::Arc;

use crate::api::login as login_api;
use crate::api::ZaloApi;
use crate::context::{create_context, ContextSession, Credentials, Options, ZaloCookie};
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::http;
use crate::{zlog_info};

/// Entry point of the library: authenticates and returns a ready-to-use API instance.
///
/// # Example
/// ```no_run
/// # use zalo_rs::{ZaloClient, Credentials, ZaloCookie};
/// # async fn demo() -> zalo_rs::errors::Result<()> {
/// let client = ZaloClient::new(Default::default());
/// let creds = Credentials {
///     imei: "...".into(),
///     cookie: vec![/* ... */],
///     user_agent: "Mozilla/5.0 ...".into(),
///     language: Some("vi".into()),
/// };
/// let api = client.login(creds).await?;
/// # Ok(())
/// # }
/// ```
pub struct ZaloClient {
    options: Options,
}

impl ZaloClient {
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    /// Logs in with an existing cookie + imei + userAgent triple.
    pub async fn login(&self, credentials: Credentials) -> Result<ZaloApi> {
        let mut ctx = create_context(self.options.api_type, self.options.api_version);
        ctx.options = self.options.clone();
        self.login_with_context(&mut ctx, credentials).await
    }

    /// Alias for [`login`](Self::login).
    pub async fn login_from_browser(&self, credentials: Credentials) -> Result<ZaloApi> {
        self.login(credentials).await
    }

    fn parse_cookies(&self, cookies: &[ZaloCookie]) -> Arc<CookieStoreMutex> {
        let mut store = cookie_store::CookieStore::new(None);
        for c in cookies {
            let name = c.key.as_deref().unwrap_or(&c.name);
            let domain = c.domain.as_deref().unwrap_or("chat.zalo.me");
            let domain_clean = domain.strip_prefix('.').unwrap_or(domain);
            let url_str = format!("https://{}", domain_clean);
            let path = c.path.as_deref().unwrap_or("/");
            let secure = c.secure.unwrap_or(false);
            let http_only = c.http_only.unwrap_or(false);
            // Build a Set-Cookie string that the cookie_store can parse
            let mut cookie_str = format!("{}={}; Domain={}; Path={}", name, c.value, domain_clean, path);
            if secure {
                cookie_str.push_str("; Secure");
            }
            if http_only {
                cookie_str.push_str("; HttpOnly");
            }
            if let Ok(url) = url::Url::parse(&url_str) {
                let _ = store.parse(&cookie_str, &url);
            }
        }
        Arc::new(CookieStoreMutex::new(store))
    }

    async fn login_with_context(
        &self,
        ctx: &mut crate::context::ContextBase,
        credentials: Credentials,
    ) -> Result<ZaloApi> {
        if credentials.imei.is_empty()
            || credentials.cookie.is_empty()
            || credentials.user_agent.is_empty()
        {
            return Err(ZaloError::Api(ZaloApiError::new("Missing required params")));
        }

        ctx.imei = Some(credentials.imei.clone());
        ctx.user_agent = Some(credentials.user_agent.clone());
        ctx.language = Some(credentials.language.clone().unwrap_or_else(|| "vi".into()));

        let cookie_store = self.parse_cookies(&credentials.cookie);
        http::init_http(ctx, cookie_store).await?;

        let login_data = login_api::login(ctx, true).await?;
        let server_info = login_api::get_server_info(ctx, true).await?;

        let login_data = login_data.ok_or_else(|| {
            ZaloError::Api(ZaloApiError::new("Đăng nhập thất bại: empty login response"))
        })?;

        // login_data may be wrapped in { data: ... } or be the data directly
        let login_info_val = login_data.get("data").cloned().unwrap_or(login_data);

        let login_info: crate::context::LoginInfo =
            serde_json::from_value(login_info_val.clone()).unwrap_or_default();

        let secret_key = login_info
            .zpw_enk
            .clone()
            .or_else(|| login_info_val.get("zpw_enk").and_then(|v| v.as_str()).map(String::from))
            .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Missing zpw_enk in login response")))?;

        let uid = login_info
            .uid
            .clone()
            .or_else(|| login_info_val.get("uid").and_then(|v| v.as_str()).map(String::from))
            .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Missing uid in login response")))?;

        ctx.secret_key = Some(secret_key);
        ctx.uid = Some(uid.clone());
        ctx.login_info = Some(login_info.clone());

        // Settings: Zalo sometimes returns `setttings` (typo) instead of `settings`
        let settings = server_info
            .settings
            .clone()
            .or(server_info.setttings.clone());
        ctx.settings = settings;
        ctx.extra_ver = server_info.extra_ver.clone();

        if crate::context::ContextSession::from_base(ctx).is_none() {
            return Err(ZaloError::Api(ZaloApiError::new("Khởi tạo ngữ cảnh thất bại")));
        }

        zlog_info!("Logged in as {}", uid);

        let session = ContextSession::from_base(ctx)
            .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to create session context")))?;

        let ws_urls = login_info.zpw_ws.clone().unwrap_or_default();
        let zpw_service_map = login_info.zpw_service_map_v3.clone().unwrap_or(serde_json::Value::Null);

        Ok(ZaloApi::new(session, zpw_service_map, ws_urls))
    }
}
