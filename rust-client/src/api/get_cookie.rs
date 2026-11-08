//! Get cookie string.

use crate::context::ContextSession;
use crate::errors::Result;

pub type GetCookieResponse = String;

/// Get the current cookie string.
pub fn get_cookie(_ctx: &ContextSession) -> Result<GetCookieResponse> {
    Ok(String::new())
}
