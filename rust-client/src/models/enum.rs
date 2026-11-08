//! Shared enumerations used across Zalo models.

use serde::{Deserialize, Serialize};

/// Thread type: user (0) or group (1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreadType {
    User = 0,
    Group = 1,
}

/// Destination type used in API requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DestType {
    Group = 1,
    User = 3,
    Page = 5,
}

/// Gender of a user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male = 0,
    Female = 1,
}

/// Avatar size options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvatarSize {
    Small = 120,
    /// Use only if you know what you're doing.
    Medium = 180,
    Large = 240,
    /// Use only if you know what you're doing.
    ExtraLarge = 360,
}
