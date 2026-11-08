//! Reaction types.

use serde::{Deserialize, Serialize};


/// Emoji reaction codes used by Zalo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reactions {
    #[serde(rename = "/-heart")]
    Heart,
    #[serde(rename = "/-strong")]
    Like,
    #[serde(rename = ":>")]
    Haha,
    #[serde(rename = ":o")]
    Wow,
    #[serde(rename = ":-(((")]
    Cry,
    #[serde(rename = ":-h")]
    Angry,
    #[serde(rename = ":-*")]
    Kiss,
    #[serde(rename = ":')")]
    TearsOfJoy,
    #[serde(rename = "/-shit")]
    Shit,
    #[serde(rename = "/-rose")]
    Rose,
    #[serde(rename = "/-break")]
    BrokenHeart,
    #[serde(rename = "/-weak")]
    Dislike,
    #[serde(rename = ";xx")]
    Love,
    #[serde(rename = ":-/")]
    Confused,
    #[serde(rename = ";-)")]
    Wink,
    #[serde(rename = "/-fade")]
    Fade,
    #[serde(rename = "/-li")]
    Sun,
    #[serde(rename = "/-bd")]
    Birthday,
    #[serde(rename = "/-bome")]
    Bomb,
    #[serde(rename = "/-ok")]
    Ok,
    #[serde(rename = "/-v")]
    Peace,
    #[serde(rename = "/-thanks")]
    Thanks,
    #[serde(rename = "/-punch")]
    Punch,
    #[serde(rename = "/-share")]
    Share,
    #[serde(rename = "_()_")]
    Pray,
    #[serde(rename = "/-no")]
    No,
    #[serde(rename = "/-bad")]
    Bad,
    #[serde(rename = "/-loveu")]
    LoveYou,
    #[serde(rename = "--b")]
    Sad,
    #[serde(rename = ":((")]
    VerySad,
    #[serde(rename = "x-)")]
    Cool,
    #[serde(rename = "8-)")]
    Nerd,
    #[serde(rename = ";-d")]
    BigSmile,
    #[serde(rename = "b-)")]
    Sunglasses,
    #[serde(rename = ":--|")]
    Neutral,
    #[serde(rename = "p-(")]
    SadFace,
    #[serde(rename = ":-bye")]
    Bye,
    #[serde(rename = "|-)")]
    Sleepy,
    #[serde(rename = ":wipe")]
    Wipe,
    #[serde(rename = ":-dig")]
    Dig,
    #[serde(rename = "&-(")]
    Anguish,
    #[serde(rename = ":handclap")]
    Handclap,
    #[serde(rename = ">-|")]
    AngryFace,
    #[serde(rename = ":-f")]
    FChair,
    #[serde(rename = ":-l")]
    LChair,
    #[serde(rename = ":-r")]
    RChair,
    #[serde(rename = ";-x")]
    Silent,
    #[serde(rename = ":-o")]
    Surprise,
    #[serde(rename = ";-s")]
    Embarrassed,
    #[serde(rename = ";-a")]
    Afraid,
    #[serde(rename = ":-<")]
    Sad2,
    #[serde(rename = ":))")]
    BigLaugh,
    #[serde(rename = "$-)")]
    Rich,
    #[serde(rename = "/-beer")]
    Beer,
    #[serde(rename = "")]
    None,
}

/// Core reaction data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TReaction {
    #[serde(rename = "actionId")]
    pub action_id: String,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "cliMsgId")]
    pub cli_msg_id: String,
    #[serde(rename = "msgType")]
    pub msg_type: String,
    #[serde(rename = "uidFrom")]
    pub uid_from: String,
    #[serde(rename = "idTo")]
    pub id_to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dName")]
    pub d_name: Option<String>,
    pub content: ReactionContent,
    pub ts: String,
    pub ttl: i64,
}

/// Content of a reaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionContent {
    #[serde(rename = "rMsg")]
    pub r_msg: Vec<ReactionMessageRef>,
    #[serde(rename = "rIcon")]
    pub r_icon: Reactions,
    #[serde(rename = "rType")]
    pub r_type: i64,
    pub source: i64,
}

/// A referenced message in a reaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionMessageRef {
    #[serde(rename = "gMsgID")]
    pub g_msg_id: String,
    #[serde(rename = "cMsgID")]
    pub c_msg_id: String,
    #[serde(rename = "msgType")]
    pub msg_type: i64,
}

/// A reaction event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub data: TReaction,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
    #[serde(rename = "isGroup")]
    pub is_group: bool,
}
