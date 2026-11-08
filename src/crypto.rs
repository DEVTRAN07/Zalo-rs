//! Cryptography helpers for the Zalo protocol.
//!
//! Implements MD5 signing, AES-CBC (zero IV, PKCS7) parameter encryption,
//! response decryption, and the `ParamsEncryptor` used by login flows.

use aes::Aes128;
use aes::Aes192;
use aes::Aes256;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use md5::{Digest, Md5};
use rand::Rng;
use serde_json::Value;
use uuid::Uuid;

use crate::errors::ZaloApiError;

const ZERO_IV: [u8; 16] = [0u8; 16];

/// LocalStorage keys that may contain the Zalo device IMEI/identifier.
pub const ZALO_IMEI_STORAGE_KEYS: &[&str] = &["z_uuid", "sh_z_uuid"];

/// Compute MD5 hash and return lowercase hex string.
pub fn md5_hex(data: impl AsRef<[u8]>) -> String {
    let mut hasher = Md5::new();
    hasher.update(data.as_ref());
    hex::encode(hasher.finalize())
}

/// Compute signed key (`signkey`) required by Zalo API requests.
///
/// `signkey = MD5("zsecure" + type + sorted param values)`
pub fn get_sign_key(api_type: &str, params: &serde_json::Map<String, Value>) -> String {
    let mut keys: Vec<&String> = params.keys().collect();
    keys.sort();
    let mut source = format!("zsecure{}", api_type);
    for key in keys {
        if let Some(v) = params.get(key) {
            match v {
                Value::String(s) => source.push_str(s),
                Value::Number(n) => source.push_str(&n.to_string()),
                Value::Bool(b) => source.push_str(if *b { "true" } else { "false" }),
                Value::Null => {}
                other => source.push_str(&other.to_string()),
            }
        }
    }
    md5_hex(source)
}

fn encrypt_aes_cbc_raw(key: &[u8], plaintext: &[u8]) -> Option<Vec<u8>> {
    match key.len() {
        16 => {
            let enc = cbc::Encryptor::<Aes128>::new(key.into(), &ZERO_IV.into());
            Some(enc.encrypt_padded_vec_mut::<Pkcs7>(plaintext))
        }
        24 => {
            let enc = cbc::Encryptor::<Aes192>::new(key.into(), &ZERO_IV.into());
            Some(enc.encrypt_padded_vec_mut::<Pkcs7>(plaintext))
        }
        32 => {
            let enc = cbc::Encryptor::<Aes256>::new(key.into(), &ZERO_IV.into());
            Some(enc.encrypt_padded_vec_mut::<Pkcs7>(plaintext))
        }
        _ => None,
    }
}

fn decrypt_aes_cbc_raw(key: &[u8], ciphertext: &[u8]) -> Option<Vec<u8>> {
    match key.len() {
        16 => {
            let dec = cbc::Decryptor::<Aes128>::new(key.into(), &ZERO_IV.into());
            dec.decrypt_padded_vec_mut::<Pkcs7>(ciphertext).ok()
        }
        24 => {
            let dec = cbc::Decryptor::<Aes192>::new(key.into(), &ZERO_IV.into());
            dec.decrypt_padded_vec_mut::<Pkcs7>(ciphertext).ok()
        }
        32 => {
            let dec = cbc::Decryptor::<Aes256>::new(key.into(), &ZERO_IV.into());
            dec.decrypt_padded_vec_mut::<Pkcs7>(ciphertext).ok()
        }
        _ => None,
    }
}

/// Encrypts a payload with an AES-CBC key (zero IV, PKCS7 padding).
///
/// `secret_key` is base64-encoded. Returns base64 ciphertext, or `None` on failure.
pub fn encode_aes(secret_key_b64: &str, data: impl AsRef<[u8]>) -> Option<String> {
    let key = BASE64.decode(secret_key_b64).ok()?;
    let encrypted = encrypt_aes_cbc_raw(&key, data.as_ref())?;
    Some(BASE64.encode(encrypted))
}

/// Decrypts a Base64 AES-CBC ciphertext (zero IV, PKCS7 padding).
///
/// `secret_key` is base64-encoded. Returns decrypted UTF-8 string, or `None` on failure.
pub fn decode_aes(secret_key_b64: &str, data_b64: &str) -> Option<String> {
    let key = BASE64.decode(secret_key_b64).ok()?;
    let ciphertext = BASE64.decode(data_b64).ok()?;
    let decrypted = decrypt_aes_cbc_raw(&key, &ciphertext)?;
    String::from_utf8(decrypted).ok()
}

/// Decrypts with a UTF-8 key (not base64) and URL-decodes the ciphertext first.
/// Used for login response decryption with `enk`.
fn decode_with_utf8_key(key: &str, data: &str) -> String {
    let decoded_url = urlencoding::decode(data).unwrap_or_else(|_| data.into());
    let ciphertext = match BASE64.decode(decoded_url.as_ref()) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    let key_bytes = key.as_bytes();
    match decrypt_aes_cbc_raw(key_bytes, &ciphertext) {
        Some(bytes) => String::from_utf8(bytes).unwrap_or_default(),
        None => String::new(),
    }
}

/// Decrypts and parses an encrypted login response.
///
/// `key` is the UTF-8 encryption key (enk) from the login param encryptor.
/// Returns parsed JSON value, or the raw string if JSON parsing fails.
pub fn decrypt_resp(key: &str, data: &str) -> Option<Value> {
    let decoded = decode_with_utf8_key(key, data);
    if decoded.is_empty() {
        return None;
    }
    match serde_json::from_str(&decoded) {
        Ok(v) => Some(v),
        Err(_) => Some(Value::String(decoded)),
    }
}

/// Encrypts payload with a raw UTF-8 AES key (used by ParamsEncryptor).
/// Returns hex or base64 encoded ciphertext.
pub fn encode_aes_utf8_key(
    key: &str,
    message: &str,
    encoding: &str,
    uppercase: bool,
) -> Option<String> {
    if message.is_empty() {
        return None;
    }
    let key_bytes = key.as_bytes();
    let encrypted = encrypt_aes_cbc_raw(key_bytes, message.as_bytes())?;
    let mut output = match encoding {
        "hex" => hex::encode(&encrypted),
        _ => BASE64.encode(&encrypted),
    };
    if uppercase {
        output = output.to_uppercase();
    }
    Some(output)
}

fn random_string(min: usize, max: usize) -> String {
    let mut rng = rand::thread_rng();
    let upper = if max > min { max } else { 12 };
    let length = rng.gen_range(min..=upper);
    if length > 12 {
        let mut result = String::new();
        let mut remaining = length;
        while remaining > 0 {
            let chunk = remaining.min(12);
            let v: u64 = rng.gen();
            result.push_str(&format!("{:x}", v)[..chunk]);
            remaining -= chunk;
        }
        result.truncate(length);
        return result;
    }
    let v: u128 = rng.gen();
    let s = format!("{:032x}", v);
    s[..length].to_string()
}

fn process_str(value: &str) -> (Option<Vec<char>>, Option<Vec<char>>) {
    if value.is_empty() {
        return (None, None);
    }
    let mut even = Vec::new();
    let mut odd = Vec::new();
    for (i, ch) in value.chars().enumerate() {
        if i % 2 == 0 {
            even.push(ch);
        } else {
            odd.push(ch);
        }
    }
    (Some(even), Some(odd))
}

/// Encrypts login params the way the Zalo web client does.
///
/// Derives a device identifier (`zcid`) and an encryption key from it.
pub struct ParamsEncryptor {
    zcid: Option<String>,
    enc_ver: String,
    zcid_ext: String,
    encrypt_key: Option<String>,
}

impl ParamsEncryptor {
    /// Create a new `ParamsEncryptor`.
    ///
    /// `api_type` corresponds to `ctx.API_TYPE`, `imei` is the device ID,
    /// and `first_launch_time` is typically `Date.now()` in milliseconds.
    pub fn new(api_type: u32, imei: &str, first_launch_time: i64) -> Result<Self, ZaloApiError> {
        let mut enc = Self {
            zcid: None,
            enc_ver: "v2".to_string(),
            zcid_ext: random_string(6, 12),
            encrypt_key: None,
        };
        enc.create_zcid(api_type, imei, first_launch_time)?;
        enc.create_encrypt_key(0);
        Ok(enc)
    }

    pub fn get_encrypt_key(&self) -> Result<&str, ZaloApiError> {
        self.encrypt_key
            .as_deref()
            .ok_or_else(|| ZaloApiError::new("getEncryptKey: didn't create encryptKey yet"))
    }

    fn create_zcid(
        &mut self,
        api_type: u32,
        imei: &str,
        first_launch_time: i64,
    ) -> Result<(), ZaloApiError> {
        if imei.is_empty() || first_launch_time == 0 {
            return Err(ZaloApiError::new("createZcid: missing params"));
        }
        let msg = format!("{},{},{}", api_type, imei, first_launch_time);
        let zcid = encode_aes_utf8_key("3FC4F0D2AB50057BCE0D90D9187A22B1", &msg, "hex", true)
            .ok_or_else(|| ZaloApiError::new("createZcid: AES encryption failed"))?;
        self.zcid = Some(zcid);
        Ok(())
    }

    fn create_encrypt_key(&mut self, attempt: u32) -> bool {
        let zcid = match &self.zcid {
            Some(v) => v.clone(),
            None => return false,
        };
        let hashed = md5_hex(&self.zcid_ext).to_uppercase();

        let derive = |even_source: &str, odd_source: &str| -> Option<String> {
            let (even, _) = process_str(even_source);
            let (even_odd, odd) = process_str(odd_source);
            let even = even?;
            let even_odd = even_odd?;
            let mut odd = odd?;
            if even.is_empty() || even_odd.is_empty() || odd.is_empty() {
                return None;
            }
            odd.reverse();
            let part1: String = even.iter().take(8).collect();
            let part2: String = even_odd.iter().take(12).collect();
            let part3: String = odd.iter().take(12).collect();
            Some(format!("{}{}{}", part1, part2, part3))
        };

        if let Some(key) = derive(&hashed, &zcid) {
            self.encrypt_key = Some(key);
            return true;
        }
        if attempt < 3 {
            return self.create_encrypt_key(attempt + 1);
        }
        false
    }

    /// Returns the params object to send with login requests.
    pub fn get_params(&self) -> Option<serde_json::Value> {
        let zcid = self.zcid.as_ref()?;
        Some(serde_json::json!({
            "zcid": zcid,
            "zcid_ext": self.zcid_ext,
            "enc_ver": self.enc_ver,
        }))
    }
}

/// Generates a device identifier (`imei`) bound to the given user agent.
///
/// # Deprecated
/// Prefer extracting the real IMEI from browser storage via
/// `extract_imei_from_storage` (keys `z_uuid` / `sh_z_uuid`) when available.
#[deprecated(note = "Prefer extracting the real IMEI from browser storage")]
pub fn generate_zalo_uuid(user_agent: &str) -> String {
    format!("{}-{}", Uuid::new_v4(), md5_hex(user_agent))
}

/// Simple trait for reading key-value storage (e.g., localStorage).
pub trait GetItem {
    fn get_item(&self, key: &str) -> Option<String>;
}

/// Extracts the Zalo device IMEI/identifier from a browser-like storage object.
///
/// Zalo stores the IMEI in `localStorage` under `z_uuid` or `sh_z_uuid`.
pub fn extract_imei_from_storage(storage: &dyn GetItem) -> Option<String> {
    for key in ZALO_IMEI_STORAGE_KEYS {
        if let Some(value) = storage.get_item(key) {
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}

/// Hashes a 4-digit PIN into the 32-character hex digest used by Zalo.
pub fn encrypt_pin(pin: &str) -> String {
    md5_hex(pin)
}

/// Verifies a PIN against its stored hash.
pub fn validate_pin(encrypted_pin: &str, pin: &str) -> bool {
    md5_hex(pin) == encrypted_pin
}
