//! Bank account and BIN types.

use serde::{Deserialize, Serialize};

/// Bank information returned by the bank list API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankInfo {
    pub bin: i64,
    pub logo: String,
    pub name: String,
    #[serde(rename = "name_eng")]
    pub name_eng: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
    #[serde(rename = "search_key_word")]
    pub search_key_word: String,
}

/// A user's bank account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: String,
    pub bin: i64,
    #[serde(default)]
    pub default: bool,
    #[serde(rename = "bank_number")]
    pub bank_number: String,
    /// Not returned by `getBankAccounts` / `createBankAccount`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_logo: Option<String>,
    #[serde(rename = "holder_name")]
    pub holder_name: String,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
    #[serde(rename = "account_id")]
    pub account_id: i64,
    /// Not returned by `getBankAccounts` / `createBankAccount`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "is_default")]
    pub is_default: bool,
}

/// Bank card BIN codes observed from Zalo mobile/web and partner banks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinBankCard {
    /// NH TMCP An Binh
    ABBank = 970425,
    /// NH TMCP Au Chau
    ACB = 970416,
    /// NH Nong nghiep va Phat tieng Nong thon Viet Nam
    Agribank = 970405,
    /// NH TMCP Dau tu va Phat tieng Viet Nam
    BIDV = 970418,
    /// Ngan hang BNP Paribas - CN TP. Ho Chi Minh
    BNPParibasHCM = 963666,
    /// Ngan hang BNP Paribas - CN Ha Noi
    BNPParibasHN = 963668,
    /// NH TMCP Ban Viet
    BVBank = 970454,
    /// NH TMCP Bac A
    BacABank = 970409,
    /// NH TMCP Bao Viet
    BaoVietBank = 970438,
    /// NH so CAKE by VPBank - TMCP Viet Nam Thinh Vuong
    CAKE = 546034,
    /// Ngan hang Cathay United - CN TP. Ho Chi Minh
    CathayUnitedHCM = 168999,
    /// NH Thuong mai TNHH MTV Xay dung Viet Nam
    VCBNeo = 970444,
    /// NH TNHH MTV CIMB Viet Nam
    CIMBBank = 422589,
    /// NH Hop tac xa Viet Nam
    CoopBank = 970446,
    /// NH TNHH MTV Phat tieng Singapore - CN TP. Ho Chi Minh
    DBSBank = 796500,
    /// NH TMCP Dong A
    DongABank = 970406,
    /// NH TMCP Xuat Nhap khau Viet Nam
    Eximbank = 970431,
    /// Ngan hang Citibank Viet Nam
    Citibank = 533948,
    /// NH TMCP Dau khi Toan cau
    GPBank = 970408,
    /// NH TMCP Phat tieng TP. Ho Chi Minh
    HDBank = 970437,
    /// NH TNHH MTV HSBC (Viet Nam)
    HSBC = 458761,
    /// NH TNHH MTV Hong Leong Viet Nam
    HongLeongBank = 970442,
    /// NH Cong nghiep Han Quoc - CN TP. Ho Chi Minh
    IBKHCM = 970456,
    /// NH Cong nghiep Han Quoc - CN Ha Noi
    IBKHN = 970455,
    /// NH TNHH Indovina
    IndovinaBank = 970434,
    /// NH Dai chung TNHH Kasikornbank - CN TP. Ho Chi Minh
    KBank = 668888,
    /// NH TMCP Kien Long
    KienlongBank = 970452,
    /// NH Kookmin - CN TP. Ho Chi Minh
    KookminBankHCM = 970463,
    /// NH Kookmin - CN Ha Noi
    KookminBankHN = 970462,
    /// Liobank by OCB
    Liobank = 963369,
    /// NH TMCP Loc Phat Viet Nam
    LPBank = 970449,
    /// NH TMCP Quan doi
    MBBank = 970422,
    /// NH TMCP Hang Hai
    MSB = 970426,
    /// MoMo
    MoMo = 971025,
    /// NH TMCP Quoc Dan
    NCB = 970419,
    /// NH TMCP Nam A
    NamABank = 970428,
    /// NH Nonghyup - CN Ha Noi
    NongHyupBank = 801011,
    /// NH TMCP Phuong Dong
    OCB = 970448,
    /// NH Thuong mai TNHH MTV Dai Duong
    OceanBank = 970414,
    /// NH TMCP Thinh vuong va Phat tieng
    PGBank = 970430,
    /// NH TMCP Dai Chung Viet Nam
    PVcomBank = 970412,
    /// NH TNHH MTV Public Viet Nam
    PublicBankVietnam = 970439,
    /// NH TMCP Sai Gon
    SCB = 970429,
    /// NH TMCP Sai Gon - Ha Noi
    SHB = 970443,
    /// NH TMCP Sai Gon Thuong Tin
    Sacombank = 970403,
    /// NH TMCP Sai Gon Cong Thuong
    SaigonBank = 970400,
    /// NH TMCP Dong Nam A
    SeABank = 970440,
    /// NH TNHH MTV Shinhan Viet Nam
    ShinhanBank = 970424,
    /// NH TNHH MTV Standard Chartered Bank Viet Nam
    StandardCharteredVietnam = 970410,
    /// NH so TNEX
    TNEX = 963326,
    /// NH TMCP Tien Phong
    TPBank = 970423,
    /// NH TMCP Ky thuong Viet Nam
    Techcombank = 970407,
    /// NH so Timo by Ban Viet Bank
    Timo = 963388,
    /// NH so UBank by VPBank
    UBank = 546035,
    /// NH United Overseas Bank Viet Nam
    UnitedOverseasBankVietnam = 970458,
    /// NH TMCP Quoc te Viet Nam
    VIB = 970441,
    /// NH TMCP Viet Nam Thinh Vuong
    VPBank = 970432,
    /// NH Lien doanh Viet - Nga
    VRB = 970421,
    /// NH TMCP Viet A
    VietABank = 970427,
    /// NH TMCP Viet Nam Thuong Tin
    VietBank = 970433,
    /// NH TMCP Ngoai Thuong Viet Nam
    Vietcombank = 970436,
    /// NH TMCP Cong thuong Viet Nam
    VietinBank = 970415,
    /// NH TNHH MTV Woori Viet Nam
    WooriBank = 970457,
}
