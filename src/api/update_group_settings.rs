use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateGroupSettingsOptions {
    /// Disallow group members to change the group name and avatar.
    #[serde(default, rename = "blockName")]
    pub block_name: bool,
    /// Highlight messages from owner/admins.
    #[serde(default, rename = "signAdminMsg")]
    pub sign_admin_msg: bool,
    /// Don't pin messages, notes, and polls to the top of a conversation.
    #[serde(default, rename = "setTopicOnly")]
    pub set_topic_only: bool,
    /// Allow new members to read most recent messages.
    #[serde(default, rename = "enableMsgHistory")]
    pub enable_msg_history: bool,
    /// Membership approval.
    #[serde(default, rename = "joinAppr")]
    pub join_appr: bool,
    /// Disallow group members to create notes & reminders.
    #[serde(default, rename = "lockCreatePost")]
    pub lock_create_post: bool,
    /// Disallow group members to create polls.
    #[serde(default, rename = "lockCreatePoll")]
    pub lock_create_poll: bool,
    /// Disallow group members to send messages.
    #[serde(default, rename = "lockSendMsg")]
    pub lock_send_msg: bool,
    /// Disallow group members to view full member list (community only).
    #[serde(default, rename = "lockViewMember")]
    pub lock_view_member: bool,
}

pub type UpdateGroupSettingsResponse = Value;

/// Update group settings.
///
/// Note: Zalo might throw an error with code 166 if you don't have enough permissions.
pub async fn update_group_settings(
    client: &ApiClient,
    options: UpdateGroupSettingsOptions,
    group_id: &str,
) -> Result<UpdateGroupSettingsResponse> {
    let params = serde_json::json!({
        "blockName": if options.block_name { 1 } else { 0 },
        "signAdminMsg": if options.sign_admin_msg { 1 } else { 0 },
        "setTopicOnly": if options.set_topic_only { 1 } else { 0 },
        "enableMsgHistory": if options.enable_msg_history { 1 } else { 0 },
        "joinAppr": if options.join_appr { 1 } else { 0 },
        "lockCreatePost": if options.lock_create_post { 1 } else { 0 },
        "lockCreatePoll": if options.lock_create_poll { 1 } else { 0 },
        "lockSendMsg": if options.lock_send_msg { 1 } else { 0 },
        "lockViewMember": if options.lock_view_member { 1 } else { 0 },
        "bannFeature": 0,
        "dirtyMedia": 0,
        "banDuration": 0,
        "blocked_members": [],
        "grid": group_id,
        "imei": client.ctx().imei,
    });

    let base_url = client.service_url("group", "/api/group/setting/update")?;
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    client.resolve(res).await
}
