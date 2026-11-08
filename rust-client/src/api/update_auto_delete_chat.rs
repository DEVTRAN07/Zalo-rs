use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum ChatTTL {
    NoDelete = 0,
    OneDay = 86_400_000,
    SevenDays = 604_800_000,
    FourteenDays = 1_209_600_000,
}

pub type UpdateAutoDeleteChatResponse = Value;

/// Update auto-delete chat settings.
pub async fn update_auto_delete_chat(
    client: &ApiClient,
    ttl: ChatTTL,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<UpdateAutoDeleteChatResponse> {
    let params = serde_json::json!({
        "threadId": thread_id,
        "isGroup": if matches!(thread_type, ThreadType::Group) { 1 } else { 0 },
        "ttl": ttl as i64,
        "clientLang": client.ctx().language,
    });

    let url = client.service_url("conversation", "/api/conv/autodelete/updateConvers")?;
    client.post_encrypted(&url, &params).await
}
