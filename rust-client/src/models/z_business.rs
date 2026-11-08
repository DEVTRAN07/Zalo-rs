//! Zalo business types.

use serde::{Deserialize, Serialize};

/// A Zalo business package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZBusinessPackage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(rename = "pkgId")]
    pub pkg_id: i64,
}

/// Business category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusinessCategory {
    Other = 0,
    RealEstate = 1,
    TechnologyAndDevices = 2,
    TravelAndHospitality = 3,
    EducationAndTraining = 4,
    ShoppingAndRetail = 5,
    CosmeticsAndBeauty = 6,
    RestaurantAndCafe = 7,
    AutoAndMotorbike = 8,
    FashionAndApparel = 9,
    FoodAndBeverage = 10,
    MediaAndEntertainment = 11,
    InternalCommunications = 12,
    Transportation = 13,
    Telecommunications = 14,
}

/// Business category names (Vietnamese).
pub fn business_category_name(category: BusinessCategory) -> &'static str {
    match category {
        BusinessCategory::Other => "Dich vu khac (Khong hien thi)",
        BusinessCategory::RealEstate => "Bat dong san",
        BusinessCategory::TechnologyAndDevices => "Cong nghe & Thiet bi",
        BusinessCategory::TravelAndHospitality => "Du ly & Luu tru",
        BusinessCategory::EducationAndTraining => "Giao duc & Dao tao",
        BusinessCategory::ShoppingAndRetail => "Mua sam & Ban le",
        BusinessCategory::CosmeticsAndBeauty => "My pham & Lam dep",
        BusinessCategory::RestaurantAndCafe => "Nha hang & Quan",
        BusinessCategory::AutoAndMotorbike => "O to & Xe may",
        BusinessCategory::FashionAndApparel => "Thoi trang & May mac",
        BusinessCategory::FoodAndBeverage => "Thuc pham & Do uong",
        BusinessCategory::MediaAndEntertainment => "Truyen thong & Giai tri",
        BusinessCategory::InternalCommunications => "Truyen thong noi bo",
        BusinessCategory::Transportation => "Van tai",
        BusinessCategory::Telecommunications => "Vien thong",
    }
}
