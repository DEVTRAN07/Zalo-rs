use zalo_rs::crypto::{
    decode_aes, encode_aes, encode_aes_utf8_key, encrypt_pin, extract_imei_from_storage,
    get_sign_key, md5_hex, validate_pin, GetItem, ParamsEncryptor, ZALO_IMEI_STORAGE_KEYS,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::{json, Map, Value};

#[test]
fn md5_hex_matches_known_values() {
    assert_eq!(md5_hex(""), "d41d8cd98f00b204e9800998ecf8427e");
    assert_eq!(md5_hex("hello"), "5d41402abc4b2a76b9719d911017c592");
    assert_eq!(md5_hex("zalo"), md5_hex(b"zalo"));
}

#[test]
fn encode_decode_aes_round_trip_base64_key() {
    // 16-byte key for AES-128, base64-encoded
    let key = BASE64.encode([0x11u8; 16]);
    let plaintext = r#"{"hello":"world","n":123}"#;
    let enc = encode_aes(&key, plaintext).expect("encrypt should succeed");
    assert_ne!(enc, plaintext);
    let dec = decode_aes(&key, &enc).expect("decrypt should succeed");
    assert_eq!(dec, plaintext);
}

#[test]
fn get_sign_key_is_deterministic_and_sorts_params() {
    let mut params_a = Map::new();
    params_a.insert("b".to_string(), json!("2"));
    params_a.insert("a".to_string(), json!("1"));
    params_a.insert("n".to_string(), json!(42));
    params_a.insert("flag".to_string(), json!(true));

    let mut params_b = Map::new();
    params_b.insert("flag".to_string(), json!(true));
    params_b.insert("n".to_string(), json!(42));
    params_b.insert("a".to_string(), json!("1"));
    params_b.insert("b".to_string(), json!("2"));

    let k1 = get_sign_key("testType", &params_a);
    let k2 = get_sign_key("testType", &params_b);
    assert_eq!(k1, k2);
    assert_eq!(k1.len(), 32);
    assert!(k1.chars().all(|c| c.is_ascii_hexdigit()));

    // Changing a value changes the sign key
    params_b.insert("a".to_string(), json!("999"));
    assert_ne!(k1, get_sign_key("testType", &params_b));
}

#[test]
fn params_encryptor_creates_zcid_and_encrypt_key() {
    let enc = ParamsEncryptor::new(30, "test-imei-123", 1_700_000_000_000)
        .expect("encryptor should initialize");
    let key = enc.get_encrypt_key().expect("encrypt key should exist");
    assert!(!key.is_empty());

    let params = enc.get_params().expect("params should exist");
    let zcid = params.get("zcid").and_then(Value::as_str).expect("zcid");
    let zcid_ext = params
        .get("zcid_ext")
        .and_then(Value::as_str)
        .expect("zcid_ext");
    let enc_ver = params
        .get("enc_ver")
        .and_then(Value::as_str)
        .expect("enc_ver");

    assert!(!zcid.is_empty());
    assert!(!zcid_ext.is_empty());
    assert_eq!(enc_ver, "v2");
    // zcid is produced as uppercase hex
    assert!(zcid.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn encode_aes_utf8_key_hex_and_base64() {
    // AES-128 key (16 bytes UTF-8 string)
    let key = "0123456789abcdef";
    let msg = "secret message";

    let hex_enc = encode_aes_utf8_key(key, msg, "hex", false).expect("hex encrypt");
    assert!(hex_enc.chars().all(|c| c.is_ascii_hexdigit()));
    assert_ne!(hex_enc, msg);

    let hex_upper = encode_aes_utf8_key(key, msg, "hex", true).expect("hex upper");
    assert_eq!(hex_upper, hex_enc.to_uppercase());

    let b64_enc = encode_aes_utf8_key(key, msg, "base64", false).expect("b64 encrypt");
    assert!(BASE64.decode(&b64_enc).is_ok());

    // Empty message returns None
    assert!(encode_aes_utf8_key(key, "", "hex", false).is_none());
}

#[test]
fn encrypt_and_validate_pin() {
    let pin = "1234";
    let hashed = encrypt_pin(pin);
    assert_eq!(hashed.len(), 32);
    assert_eq!(hashed, md5_hex(pin));
    assert!(validate_pin(&hashed, pin));
    assert!(!validate_pin(&hashed, "0000"));
}

struct MockStorage {
    values: std::collections::HashMap<String, String>,
}

impl MockStorage {
    fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }
    fn with(mut self, key: &str, value: &str) -> Self {
        self.values.insert(key.to_string(), value.to_string());
        self
    }
}

impl GetItem for MockStorage {
    fn get_item(&self, key: &str) -> Option<String> {
        self.values.get(key).cloned()
    }
}

#[test]
fn extract_imei_from_storage_reads_known_keys() {
    assert!(ZALO_IMEI_STORAGE_KEYS.contains(&"z_uuid"));
    assert!(ZALO_IMEI_STORAGE_KEYS.contains(&"sh_z_uuid"));

    let empty = MockStorage::new();
    assert_eq!(extract_imei_from_storage(&empty), None);

    let with_z = MockStorage::new().with("z_uuid", "imei-from-z");
    assert_eq!(
        extract_imei_from_storage(&with_z).as_deref(),
        Some("imei-from-z")
    );

    let with_sh = MockStorage::new().with("sh_z_uuid", "imei-from-sh");
    assert_eq!(
        extract_imei_from_storage(&with_sh).as_deref(),
        Some("imei-from-sh")
    );

    // z_uuid takes precedence (ordered first in ZALO_IMEI_STORAGE_KEYS)
    let both = MockStorage::new()
        .with("z_uuid", "first")
        .with("sh_z_uuid", "second");
    assert_eq!(extract_imei_from_storage(&both).as_deref(), Some("first"));

    // Empty values are ignored
    let empty_val = MockStorage::new().with("z_uuid", "");
    assert_eq!(extract_imei_from_storage(&empty_val), None);
}
