//! Group event types and initializer.

use serde::{Deserialize, Serialize};

use crate::models::group::GroupSetting;

/// Group event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupEventType {
    #[serde(rename = "join_request")]
    JoinRequest,
    #[serde(rename = "join")]
    Join,
    #[serde(rename = "leave")]
    Leave,
    #[serde(rename = "remove_member")]
    RemoveMember,
    #[serde(rename = "block_member")]
    BlockMember,
    #[serde(rename = "update_setting")]
    UpdateSetting,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "new_link")]
    NewLink,
    #[serde(rename = "add_admin")]
    AddAdmin,
    #[serde(rename = "remove_admin")]
    RemoveAdmin,
    #[serde(rename = "new_pin_topic")]
    NewPinTopic,
    #[serde(rename = "update_pin_topic")]
    UpdatePinTopic,
    #[serde(rename = "reorder_pin_topic")]
    ReorderPinTopic,
    #[serde(rename = "update_board")]
    UpdateBoard,
    #[serde(rename = "remove_board")]
    RemoveBoard,
    #[serde(rename = "update_topic")]
    UpdateTopic,
    #[serde(rename = "unpin_topic")]
    UnpinTopic,
    #[serde(rename = "remove_topic")]
    RemoveTopic,
    #[serde(rename = "accept_remind")]
    AcceptRemind,
    #[serde(rename = "reject_remind")]
    RejectRemind,
    #[serde(rename = "remind_topic")]
    RemindTopic,
    #[serde(rename = "update_avatar")]
    UpdateAvatar,
    #[serde(rename = "unknown")]
    Unknown,
}

/// Update member data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEventUpdateMember {
    pub id: String,
    #[serde(rename = "dName")]
    pub d_name: String,
    pub avatar: String,
    #[serde(rename = "type")]
    pub type_: i64,
    #[serde(rename = "avatar_25")]
    pub avatar_25: String,
}

/// Group info within an event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEventGroupInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "link_expired_time")]
    pub link_expired_time: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Extra data within an event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEventExtraData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Base group event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupEventBase {
    #[serde(rename = "subType")]
    pub sub_type: i64,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[serde(rename = "updateMembers")]
    pub update_members: Vec<GroupEventUpdateMember>,
    #[serde(rename = "groupSetting")]
    pub group_setting: Option<GroupSetting>,
    #[serde(rename = "groupTopic")]
    pub group_topic: Option<crate::models::group::GroupTopic>,
    pub info: Option<GroupEventGroupInfo>,
    #[serde(rename = "extraData")]
    pub extra_data: Option<GroupEventExtraData>,
    pub time: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fullAvt")]
    pub full_avt: Option<String>,
    #[serde(rename = "isAdd")]
    pub is_add: i64,
    #[serde(rename = "hideGroupInfo")]
    pub hide_group_info: i64,
    pub version: String,
    #[serde(rename = "groupType")]
    pub group_type: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorMap")]
    pub error_map: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e2ee: Option<i64>,
}

/// Join request event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupEventJoinRequest {
    pub uids: Vec<String>,
    #[serde(rename = "totalPending")]
    pub total_pending: i64,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub time: String,
}

/// Pin topic event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupEventPinTopic {
    #[serde(rename = "oldBoardVersion")]
    pub old_board_version: i64,
    #[serde(rename = "boardVersion")]
    pub board_version: i64,
    pub topic: crate::models::group::GroupTopic,
    #[serde(rename = "actorId")]
    pub actor_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
}

/// Reorder pin topic event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupEventReorderPinTopic {
    #[serde(rename = "oldBoardVersion")]
    pub old_board_version: i64,
    #[serde(rename = "actorId")]
    pub actor_id: String,
    pub topics: Vec<ReorderPinTopicEntry>,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "boardVersion")]
    pub board_version: i64,
    pub topic: Option<crate::models::group::GroupTopic>,
}

/// Entry in a reorder pin topic list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderPinTopicEntry {
    #[serde(rename = "topicId")]
    pub topic_id: String,
    #[serde(rename = "topicType")]
    pub topic_type: i64,
}

/// Board event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupEventBoard {
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
    pub group_topic: serde_json::Value,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updateMembers")]
    pub update_members: Option<Vec<GroupEventUpdateMember>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupSetting")]
    pub group_setting: Option<GroupSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<GroupEventGroupInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extraData")]
    pub extra_data: Option<GroupEventExtraData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avt: Option<()>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fullAvt")]
    pub full_avt: Option<()>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isAdd")]
    pub is_add: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hideGroupInfo")]
    pub hide_group_info: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupType")]
    pub group_type: Option<i64>,
}

/// Remind respond event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupEventRemindRespond {
    #[serde(rename = "topicId")]
    pub topic_id: String,
    #[serde(rename = "updateMembers")]
    pub update_members: Vec<String>,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub time: String,
}

/// Remind topic event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TGroupEventRemindTopic {
    pub msg: String,
    #[serde(rename = "editorId")]
    pub editor_id: String,
    pub color: String,
    pub emoji: String,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    #[serde(rename = "editTime")]
    pub edit_time: i64,
    #[serde(rename = "type")]
    pub type_: i64,
    pub duration: i64,
    #[serde(rename = "group_id")]
    pub group_id: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    pub repeat: i64,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    pub time: i64,
    #[serde(rename = "remindType")]
    pub remind_type: i64,
}

/// Union of all group event data shapes.
#[derive(Debug, Clone, Serialize)]
pub enum TGroupEvent {
    Base(TGroupEventBase),
    JoinRequest(TGroupEventJoinRequest),
    PinTopic(TGroupEventPinTopic),
    ReorderPinTopic(TGroupEventReorderPinTopic),
    Board(TGroupEventBoard),
    RemindRespond(TGroupEventRemindRespond),
    RemindTopic(TGroupEventRemindTopic),
}

/// A structured group event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEvent {
    #[serde(rename = "type")]
    pub type_: GroupEventType,
    pub data: serde_json::Value,
    pub act: String,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
}

/// Initialize a group event from raw data, type, and action.
pub fn initialize_group_event(uid: &str, data: &TGroupEvent, type_: GroupEventType, act: &str) -> GroupEvent {
    let thread_id = match data {
        TGroupEvent::RemindTopic(t) => t.group_id.clone(),
        _ => match data {
            TGroupEvent::Base(b) => b.group_id.clone(),
            TGroupEvent::JoinRequest(j) => j.group_id.clone(),
            TGroupEvent::PinTopic(p) => p.group_id.clone(),
            TGroupEvent::ReorderPinTopic(r) => r.group_id.clone(),
            TGroupEvent::Board(b) => b.group_id.clone(),
            TGroupEvent::RemindRespond(r) => r.group_id.clone(),
            TGroupEvent::RemindTopic(_) => String::new(),
        },
    };

    match type_ {
        GroupEventType::JoinRequest => GroupEvent {
            type_,
            data: serde_json::to_value(data).unwrap_or_default(),
            act: act.to_string(),
            thread_id,
            is_self: false,
        },
        GroupEventType::NewPinTopic | GroupEventType::UnpinTopic | GroupEventType::UpdatePinTopic => {
            let is_self = match data {
                TGroupEvent::PinTopic(p) => p.actor_id == uid,
                _ => false,
            };
            GroupEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                act: act.to_string(),
                thread_id,
                is_self,
            }
        }
        GroupEventType::ReorderPinTopic => {
            let is_self = match data {
                TGroupEvent::PinTopic(p) => p.actor_id == uid,
                _ => false,
            };
            GroupEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                act: act.to_string(),
                thread_id,
                is_self,
            }
        }
        GroupEventType::UpdateBoard | GroupEventType::RemoveBoard => {
            let is_self = match data {
                TGroupEvent::Board(b) => b.source_id == uid,
                _ => false,
            };
            GroupEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                act: act.to_string(),
                thread_id,
                is_self,
            }
        }
        GroupEventType::AcceptRemind | GroupEventType::RejectRemind => {
            let is_self = match data {
                TGroupEvent::RemindRespond(r) => r.update_members.iter().any(|m| m == uid),
                _ => false,
            };
            GroupEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                act: act.to_string(),
                thread_id,
                is_self,
            }
        }
        GroupEventType::RemindTopic => {
            let is_self = match data {
                TGroupEvent::RemindTopic(t) => t.creator_id == uid,
                _ => false,
            };
            GroupEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                act: act.to_string(),
                thread_id,
                is_self,
            }
        }
        _ => {
            let base_data = match data {
                TGroupEvent::Base(b) => b,
                _ => {
                    // Fallback: treat as base-like
                    return GroupEvent {
                        type_: GroupEventType::Unknown,
                        data: serde_json::to_value(data).unwrap_or_default(),
                        act: act.to_string(),
                        thread_id,
                        is_self: false,
                    };
                }
            };
            let is_self = base_data
                .update_members
                .iter()
                .any(|m| m.id == uid)
                || base_data.source_id == uid;
            GroupEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                act: act.to_string(),
                thread_id,
                is_self,
            }
        }
    }
}
