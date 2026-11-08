//! Group types: settings, topics, info, and membership.

use serde::{Deserialize, Serialize};

/// Group settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSetting {
    #[serde(rename = "blockName")]
    pub block_name: i64,
    #[serde(rename = "signAdminMsg")]
    pub sign_admin_msg: i64,
    #[serde(rename = "addMemberOnly")]
    pub add_member_only: i64,
    #[serde(rename = "setTopicOnly")]
    pub set_topic_only: i64,
    #[serde(rename = "enableMsgHistory")]
    pub enable_msg_history: i64,
    #[serde(rename = "joinAppr")]
    pub join_appr: i64,
    #[serde(rename = "lockCreatePost")]
    pub lock_create_post: i64,
    #[serde(rename = "lockCreatePoll")]
    pub lock_create_poll: i64,
    #[serde(rename = "lockSendMsg")]
    pub lock_send_msg: i64,
    #[serde(rename = "lockViewMember")]
    pub lock_view_member: i64,
    #[serde(rename = "bannFeature")]
    pub bann_feature: i64,
    #[serde(rename = "dirtyMedia")]
    pub dirty_media: i64,
    #[serde(rename = "banDuration")]
    pub ban_duration: i64,
}

/// Type of a group topic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupTopicType {
    Note = 0,
    Message = 2,
    Poll = 3,
}

/// Parameters for a note topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopicNoteParams {
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    #[serde(rename = "global_msg_id")]
    pub global_msg_id: String,
    pub title: String,
}

/// Parameters for a text message topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopicTextMessageParams {
    #[serde(rename = "senderUid")]
    pub sender_uid: String,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    #[serde(rename = "global_msg_id")]
    pub global_msg_id: String,
    #[serde(rename = "msg_type")]
    pub msg_type: i64,
    pub title: String,
}

/// Parameters for a voice message topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopicVoiceMessageParams {
    #[serde(rename = "senderUid")]
    pub sender_uid: String,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    #[serde(rename = "global_msg_id")]
    pub global_msg_id: String,
    #[serde(rename = "msg_type")]
    pub msg_type: i64,
    pub title: String,
    pub thumb: String,
}

/// Parameters for an image message topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopicImageMessageParams {
    #[serde(rename = "senderUid")]
    pub sender_uid: String,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    #[serde(rename = "global_msg_id")]
    pub global_msg_id: String,
    #[serde(rename = "msg_type")]
    pub msg_type: i64,
    pub title: String,
    pub thumb: String,
}

/// Parameters for a video message topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopicVideoMessageParams {
    #[serde(rename = "senderUid")]
    pub sender_uid: String,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    #[serde(rename = "global_msg_id")]
    pub global_msg_id: String,
    #[serde(rename = "msg_type")]
    pub msg_type: i64,
    pub title: String,
    pub thumb: String,
}

/// Parameters for a file message topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopicFileMessageParams {
    #[serde(rename = "senderUid")]
    pub sender_uid: String,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    #[serde(rename = "global_msg_id")]
    pub global_msg_id: String,
    #[serde(rename = "msg_type")]
    pub msg_type: i64,
    pub title: String,
    pub extra: GroupTopicFileExtra,
}

/// Extra data for a file message topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopicFileExtra {
    #[serde(rename = "fileSize")]
    pub file_size: String,
    pub checksum: String,
    #[serde(rename = "checksumSha")]
    pub checksum_sha: serde_json::Value,
    #[serde(rename = "fileExt")]
    pub file_ext: String,
    pub fdata: String,
    #[serde(rename = "fType")]
    pub f_type: i64,
}

/// Parameters for a gif message topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopicGifMessageParams {
    #[serde(rename = "senderUid")]
    pub sender_uid: String,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    #[serde(rename = "global_msg_id")]
    pub global_msg_id: String,
    #[serde(rename = "msg_type")]
    pub msg_type: i64,
    pub title: String,
    pub thumb: String,
}

/// Union of group topic message parameters.
#[derive(Debug, Clone)]
pub enum GroupTopicMessageParams {
    Text(GroupTopicTextMessageParams),
    Voice(GroupTopicVoiceMessageParams),
    Image(GroupTopicImageMessageParams),
    Video(GroupTopicVideoMessageParams),
    File(GroupTopicFileMessageParams),
    Gif(GroupTopicGifMessageParams),
}

/// Parameters for a poll topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopicPollParams {
    #[serde(rename = "pollId")]
    pub poll_id: i64,
    pub title: String,
}

/// Other / unknown topic parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopicOtherParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A group topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTopic {
    #[serde(rename = "type")]
    pub type_: GroupTopicType,
    pub color: i64,
    pub emoji: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    pub duration: i64,
    pub params: serde_json::Value,
    pub id: String,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "editorId")]
    pub editor_id: String,
    #[serde(rename = "editTime")]
    pub edit_time: i64,
    pub repeat: i64,
    pub action: i64,
}

/// Type of a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupType {
    Group = 1,
    Community = 2,
}

/// Current member of a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupCurrentMem {
    pub id: String,
    #[serde(rename = "dName")]
    pub d_name: String,
    #[serde(rename = "zaloName")]
    pub zalo_name: String,
    pub avatar: String,
    #[serde(rename = "avatar_25")]
    pub avatar_25: String,
    #[serde(rename = "accountStatus")]
    pub account_status: i64,
    pub type_: i64,
}

/// Group info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfo {
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub name: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub type_: GroupType,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    pub version: String,
    pub avt: String,
    #[serde(rename = "fullAvt")]
    pub full_avt: String,
    #[serde(rename = "memberIds")]
    pub member_ids: Vec<String>,
    #[serde(rename = "adminIds")]
    pub admin_ids: Vec<String>,
    #[serde(rename = "currentMems")]
    pub current_mems: Vec<GroupCurrentMem>,
    #[serde(default)]
    pub update_mems: Vec<serde_json::Value>,
    #[serde(default)]
    pub admins: Vec<serde_json::Value>,
    #[serde(rename = "hasMoreMember")]
    pub has_more_member: i64,
    #[serde(rename = "subType")]
    pub sub_type: i64,
    #[serde(rename = "totalMember")]
    pub total_member: i64,
    #[serde(rename = "maxMember")]
    pub max_member: i64,
    pub setting: GroupSetting,
    #[serde(rename = "createdTime")]
    pub created_time: i64,
    pub visibility: i64,
    #[serde(rename = "globalId")]
    pub global_id: String,
    /// 1: True, 0: False
    pub e2ee: i64,
    #[serde(rename = "extraInfo")]
    pub extra_info: GroupExtraInfo,
}

/// Extra info inside GroupInfo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupExtraInfo {
    #[serde(rename = "enable_media_store")]
    pub enable_media_store: i64,
}
