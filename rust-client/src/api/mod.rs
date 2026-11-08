//! Zalo API surface: holds session context and exposes endpoint methods.

pub mod get_user_info;
pub mod keep_alive;
pub mod login;
pub mod send_message;

use crate::context::ContextSession;
use crate::errors::Result;
use crate::ws::Listener;

pub use get_user_info::UserInfoResponse;
pub use keep_alive::KeepAliveResponse;

/// High-level API bound to an authenticated session.
#[derive(Clone)]
pub struct ZaloApi {
    pub(crate) ctx: ContextSession,
    pub zpw_service_map: serde_json::Value,
    ws_urls: Vec<String>,
}

impl ZaloApi {
    pub fn new(ctx: ContextSession, zpw_service_map: serde_json::Value, ws_urls: Vec<String>) -> Self {
        Self {
            ctx,
            zpw_service_map,
            ws_urls,
        }
    }

    /// Get the authenticated user ID.
    pub fn uid(&self) -> &str {
        &self.ctx.uid
    }

    /// Access the session context.
    pub fn context(&self) -> &ContextSession {
        &self.ctx
    }

    /// Get user info for one or more user IDs.
    pub async fn get_user_info(&self, user_ids: &[&str]) -> Result<UserInfoResponse> {
        get_user_info::get_user_info(&self.ctx, &self.zpw_service_map, user_ids).await
    }

    /// Keep the session alive.
    pub async fn keep_alive(&self) -> Result<KeepAliveResponse> {
        keep_alive::keep_alive(&self.ctx, &self.zpw_service_map).await
    }

    /// Send a message to a user (thread_type=0) or group (thread_type=1).
    pub async fn send_message(
        &self,
        message: &str,
        thread_id: &str,
        thread_type: u8,
    ) -> Result<serde_json::Value> {
        send_message::send_message(&self.ctx, &self.zpw_service_map, message, thread_id, thread_type).await
    }

    /// Create a WebSocket listener for real-time events.
    pub fn listener(&self) -> Listener {
        Listener::new(self.ctx.clone(), self.ws_urls.clone())
    }
}
