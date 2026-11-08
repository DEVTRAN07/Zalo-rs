//! Models: data structures for Zalo API responses and real-time events.
//!
//! All model types are `pub` and re-exported from this module. Field names use
//! Rust snake_case with `#[serde(rename = "...")]` mapping to the Zalo JSON
//! wire format (camelCase / snake_case as appropriate).

pub mod attachment;
pub mod auto_reply;
pub mod bank;
pub mod board;
pub mod catalog;
pub mod delivered_message;
pub mod r#enum;
pub mod friend_event;
pub mod group;
pub mod group_event;
pub mod label;
pub mod message;
pub mod product_catalog;
pub mod quick_message;
pub mod reaction;
pub mod reminder;
pub mod seen_message;
pub mod sticker;
pub mod typing;
pub mod undo;
pub mod user;
pub mod z_business;
