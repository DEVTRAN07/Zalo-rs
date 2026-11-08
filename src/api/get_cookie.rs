//! Get cookie string.

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type GetCookieResponse = String;

/// Get the current cookie string.
pub fn get_cookie(_client: &ApiClient) -> Result<GetCookieResponse> {
    Ok(String::new())
}
