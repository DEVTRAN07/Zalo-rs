//! Get own user ID.

use crate::context::ContextSession;
use crate::errors::Result;

pub type GetOwnIdResponse = String;

/// Get the authenticated user ID.
pub fn get_own_id(ctx: &ContextSession) -> Result<GetOwnIdResponse> {
    Ok(ctx.uid.clone())
}
