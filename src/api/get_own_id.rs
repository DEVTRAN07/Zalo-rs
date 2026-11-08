//! Get own user ID.

use crate::api::client::ApiClient;
use crate::errors::Result;

pub type GetOwnIdResponse = String;

/// Get the authenticated user ID.
pub fn get_own_id(client: &ApiClient) -> Result<GetOwnIdResponse> {
    Ok(client.ctx().uid.clone())
}
