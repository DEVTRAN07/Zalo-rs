//! `zalo-client` — Rust port of the Zalo web API protocol.
//!
//! Reverse-engineered from the TypeScript `zalo-client` implementation.
//! Provides authentication, HTTP API calls, and a WebSocket listener for
//! real-time events.

pub mod api;
pub mod client;
pub mod context;
pub mod crypto;
pub mod errors;
pub mod factory;
pub mod http;
pub mod logger;
pub mod ws;
pub mod models;
pub mod core;

pub use api::ZaloApi;
pub use client::ZaloClient;
pub use context::{Credentials, Options, RetryOptions, ZaloCookie, ZaloCredentials};
pub use errors::{Result, ZaloApiError, ZaloError};
pub use ws::{CloseReason, Listener, ListenerEvent, WsPayload};
