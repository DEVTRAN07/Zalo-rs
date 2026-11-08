//! Get context session.

use crate::context::ContextSession;
use crate::errors::Result;

pub type GetContextResponse = ContextSession;

/// Get the current context.
pub fn get_context(ctx: &ContextSession) -> Result<GetContextResponse> {
    Ok(ctx.clone())
}
