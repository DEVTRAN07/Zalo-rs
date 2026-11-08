//! Product catalog item types for Zalo Shop.

use serde::{Deserialize, Serialize};

/// A product catalog item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCatalogItem {
    pub price: String,
    pub description: String,
    /// Relative path used to build the product URL.
    ///
    /// Example: `https://catalog.zalo.me/${path}`
    pub path: String,
    #[serde(rename = "product_id")]
    pub product_id: String,
    #[serde(rename = "product_name")]
    pub product_name: String,
    #[serde(rename = "currency_unit")]
    pub currency_unit: String,
    #[serde(rename = "product_photos")]
    pub product_photos: Vec<String>,
    #[serde(rename = "create_time")]
    pub create_time: i64,
    #[serde(rename = "catalog_id")]
    pub catalog_id: String,
    #[serde(rename = "owner_id")]
    pub owner_id: String,
}
