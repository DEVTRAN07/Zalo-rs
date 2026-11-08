use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::{Result, ZaloApiError, ZaloError};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UpdateSettingsType {
    ViewBirthday,
    ShowOnlineStatus,
    DisplaySeenStatus,
    ReceiveMessage,
    AcceptCall,
    AddFriendViaPhone,
    AddFriendViaQR,
    AddFriendViaGroup,
    AddFriendViaContact,
    DisplayOnRecommendFriend,
    ArchivedChat,
    QuickMessage,
}

impl UpdateSettingsType {
    pub fn as_key(&self) -> &'static str {
        match self {
            Self::ViewBirthday => "view_birthday",
            Self::ShowOnlineStatus => "show_online_status",
            Self::DisplaySeenStatus => "display_seen_status",
            Self::ReceiveMessage => "receive_message",
            Self::AcceptCall => "accept_stranger_call",
            Self::AddFriendViaPhone => "add_friend_via_phone",
            Self::AddFriendViaQR => "add_friend_via_qr",
            Self::AddFriendViaGroup => "add_friend_via_group",
            Self::AddFriendViaContact => "add_friend_via_contact",
            Self::DisplayOnRecommendFriend => "display_on_recommend_friend",
            Self::ArchivedChat => "archivedChatStatus",
            Self::QuickMessage => "quickMessageStatus",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSettingsResponse(pub Value);

/// Update account settings.
pub async fn update_settings(
    client: &ApiClient,
    setting_type: UpdateSettingsType,
    value: i32,
) -> Result<UpdateSettingsResponse> {
    let params = serde_json::json!({ setting_type.as_key(): value });

    let base_url = "https://wpa.chat.zalo.me/api/setting/update".to_string();
    let encrypted = client
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;
    let url = client.make_url(&base_url, &[("params", encrypted)], true)?;
    let res = client.request_get(&url).await?;
    let val: Value = client.resolve(res).await?;
    Ok(UpdateSettingsResponse(val))
}
