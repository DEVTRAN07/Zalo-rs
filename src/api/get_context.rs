//! Get context session.

use crate::api::client::ApiClient;
use crate::context::ContextSession;
use crate::errors::Result;

pub type GetContextResponse = ContextSession;

/// Get the current context.
pub fn get_context(client: &ApiClient) -> Result<GetContextResponse> {
    Ok(client.ctx().clone())
}
