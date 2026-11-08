//! User types: profile, settings, and unchanged info.

use serde::{Deserialize, Serialize};

use crate::models::r#enum::Gender;
use crate::models::z_business::ZBusinessPackage;

/// A full user profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "zaloName")]
    pub zalo_name: String,
    pub avatar: String,
    #[serde(rename = "bgavatar")]
    pub bgavatar: String,
    pub cover: String,
    pub gender: Gender,
    pub dob: i64,
    pub sdob: String,
    pub status: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(rename = "isFr")]
    pub is_fr: i64,
    #[serde(rename = "isBlocked")]
    pub is_blocked: i64,
    #[serde(rename = "lastActionTime")]
    pub last_action_time: i64,
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: i64,
    #[serde(rename = "isActive")]
    pub is_active: i64,
    pub key: i64,
    #[serde(rename = "type")]
    pub type_: i64,
    #[serde(rename = "isActivePC")]
    pub is_active_pc: i64,
    #[serde(rename = "isActiveWeb")]
    pub is_active_web: i64,
    #[serde(rename = "isValid")]
    pub is_valid: i64,
    #[serde(rename = "userKey")]
    pub user_key: String,
    #[serde(rename = "accountStatus")]
    pub account_status: i64,
    #[serde(rename = "oaInfo")]
    pub oa_info: serde_json::Value,
    #[serde(rename = "user_mode")]
    pub user_mode: i64,
    #[serde(rename = "globalId")]
    pub global_id: String,
    #[serde(rename = "bizPkg")]
    pub biz_pkg: ZBusinessPackage,
    #[serde(rename = "createdTs")]
    pub created_ts: i64,
    #[serde(rename = "oa_status")]
    pub oa_status: serde_json::Value,
}

/// Basic user info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBasic {
    pub avatar: String,
    pub cover: String,
    pub status: String,
    pub gender: Gender,
    pub dob: i64,
    pub sdob: String,
    #[serde(rename = "globalId")]
    pub global_id: String,
    #[serde(rename = "bizPkg")]
    pub biz_pkg: ZBusinessPackage,
    pub uid: String,
    #[serde(rename = "zalo_name")]
    pub zalo_name: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
}

/// User settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSetting {
    #[serde(rename = "add_friend_via_contact")]
    pub add_friend_via_contact: i64,
    #[serde(rename = "display_on_recommend_friend")]
    pub display_on_recommend_friend: i64,
    #[serde(rename = "add_friend_via_group")]
    pub add_friend_via_group: i64,
    #[serde(rename = "add_friend_via_qr")]
    pub add_friend_via_qr: i64,
    #[serde(rename = "quick_message_status")]
    pub quick_message_status: i64,
    #[serde(rename = "show_online_status")]
    pub show_online_status: bool,
    #[serde(rename = "accept_stranger_call")]
    pub accept_stranger_call: i64,
    #[serde(rename = "archived_chat_status")]
    pub archived_chat_status: i64,
    #[serde(rename = "receive_message")]
    pub receive_message: i64,
    #[serde(rename = "add_friend_via_phone")]
    pub add_friend_via_phone: i64,
    #[serde(rename = "display_seen_status")]
    pub display_seen_status: i64,
    #[serde(rename = "view_birthday")]
    pub view_birthday: i64,
    #[serde(rename = "setting_2FA_status")]
    pub setting_2fa_status: i64,
}

/// Profile info that remains unchanged.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnchangedProfileInfo {
    #[serde(rename = "oa_status")]
    pub oa_status: UnchangedOaStatus,
    #[serde(rename = "isFr")]
    pub is_fr: i64,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
    #[serde(rename = "lastActionTime")]
    pub last_action_time: i64,
}

/// OA status within unchanged profile info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnchangedOaStatus {
    pub blocked: i64,
}
