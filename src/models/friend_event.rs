//! Friend event types and initializer.

use serde::{Deserialize, Serialize};

/// Friend event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FriendEventType {
    Add = 0,
    Remove = 1,
    Request = 2,
    UndoRequest = 3,
    RejectRequest = 4,
    SeenFriendRequest = 5,
    Block = 6,
    Unblock = 7,
    BlockCall = 8,
    UnblockCall = 9,
    PinUnpin = 10,
    PinCreate = 11,
    Unknown = 12,
}

/// Base friend event data (a raw string).
pub type TFriendEventBase = String;

/// Data for reject / undo request events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFriendEventRejectUndo {
    #[serde(rename = "toUid")]
    pub to_uid: String,
    #[serde(rename = "fromUid")]
    pub from_uid: String,
}

/// Data for request events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFriendEventRequest {
    #[serde(rename = "toUid")]
    pub to_uid: String,
    #[serde(rename = "fromUid")]
    pub from_uid: String,
    pub src: i64,
    pub message: String,
}

/// Data for seen friend request events (a list of uids).
pub type TFriendEventSeenRequest = Vec<String>;

/// Parameters for a pin-create topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFriendEventPinCreateTopicParams {
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

/// Pin topic reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFriendEventPinTopic {
    #[serde(rename = "topicId")]
    pub topic_id: String,
    #[serde(rename = "topicType")]
    pub topic_type: i64,
}

/// Pin create topic detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFriendEventPinCreateTopic {
    pub type_: i64,
    pub color: i64,
    pub emoji: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    pub duration: i64,
    pub params: TFriendEventPinCreateTopicParams,
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

/// Pin create event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFriendEventPinCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_topic: Option<TFriendEventPinTopic>,
    pub topic: TFriendEventPinCreateTopic,
    #[serde(rename = "actorId")]
    pub actor_id: String,
    #[serde(rename = "oldVersion")]
    pub old_version: i64,
    pub version: i64,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
}

/// Pin unpin event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFriendEventPinUnpin {
    pub topic: TFriendEventPinTopic,
    #[serde(rename = "actorId")]
    pub actor_id: String,
    #[serde(rename = "oldVersion")]
    pub old_version: i64,
    pub version: i64,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
}

/// Union of all friend event data shapes.
#[derive(Debug, Clone, Serialize)]
pub enum TFriendEvent {
    Base(String),
    Request(TFriendEventRequest),
    RejectUndo(TFriendEventRejectUndo),
    SeenRequest(TFriendEventSeenRequest),
    PinUnpin(TFriendEventPinUnpin),
    PinCreate(TFriendEventPinCreate),
}

/// A structured friend event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendEvent {
    #[serde(rename = "type")]
    pub type_: FriendEventType,
    pub data: serde_json::Value,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
}

/// Initialize a friend event from raw data and a type.
pub fn initialize_friend_event(uid: &str, data: &TFriendEvent, type_: FriendEventType) -> FriendEvent {
    use FriendEventType::*;

    let is_self = |data: &TFriendEvent, from_uid_field: bool| -> bool {
        match data {
            TFriendEvent::Request(r) => {
                if from_uid_field {
                    r.from_uid == uid
                } else {
                    r.to_uid == uid
                }
            }
            TFriendEvent::RejectUndo(ru) => ru.to_uid == uid,
            TFriendEvent::PinCreate(pc) => pc.actor_id == uid,
            TFriendEvent::PinUnpin(pu) => pu.actor_id == uid,
            _ => false,
        }
    };

    match type_ {
        Add | Remove | Block | Unblock | BlockCall | UnblockCall => {
            let thread_id = match data {
                TFriendEvent::Base(s) => s.clone(),
                _ => String::new(),
            };
            let is_self = !matches!(type_, Add | Remove);
            FriendEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                thread_id,
                is_self,
            }
        }
        RejectRequest | UndoRequest => {
            let thread_id = match data {
                TFriendEvent::RejectUndo(ru) => ru.to_uid.clone(),
                _ => String::new(),
            };
            FriendEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                thread_id,
                is_self: is_self(data, true),
            }
        }
        Request => {
            let thread_id = match data {
                TFriendEvent::Request(r) => r.to_uid.clone(),
                _ => String::new(),
            };
            FriendEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                thread_id,
                is_self: is_self(data, true),
            }
        }
        SeenFriendRequest => FriendEvent {
            type_,
            data: serde_json::to_value(data).unwrap_or_default(),
            thread_id: uid.to_string(),
            is_self: true,
        },
        PinCreate => {
            let thread_id = match data {
                TFriendEvent::PinCreate(pc) => pc.conversation_id.clone(),
                _ => String::new(),
            };
            FriendEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                thread_id,
                is_self: is_self(data, false),
            }
        }
        PinUnpin => {
            let thread_id = match data {
                TFriendEvent::PinUnpin(pu) => pu.conversation_id.clone(),
                _ => String::new(),
            };
            FriendEvent {
                type_,
                data: serde_json::to_value(data).unwrap_or_default(),
                thread_id,
                is_self: is_self(data, false),
            }
        }
        Unknown => FriendEvent {
            type_,
            data: serde_json::to_value(data).unwrap_or_default(),
            thread_id: String::new(),
            is_self: false,
        },
    }
}
