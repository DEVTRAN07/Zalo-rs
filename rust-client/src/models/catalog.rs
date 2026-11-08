//! Catalog item types for Zalo Shop.

use serde::{Deserialize, Serialize};

/// A catalog item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItem {
    pub id: String,
    pub name: String,
    pub version: i64,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    pub path: String,
    #[serde(rename = "catalogPhoto")]
    pub catalog_photo: Option<String>,
    #[serde(rename = "totalProduct")]
    pub total_product: i64,
    #[serde(rename = "created_time")]
    pub created_time: i64,
}
