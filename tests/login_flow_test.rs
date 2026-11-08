use zalo_client::crypto::{decode_aes, encode_aes, get_sign_key, ParamsEncryptor};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::{json, Map, Value};

#[test]
fn params_encryptor_full_flow() {
    let enc = ParamsEncryptor::new(30, "device-imei-abc", 1_720_000_000_000)
        .expect("ParamsEncryptor::new should succeed");
    let key = enc.get_encrypt_key().expect("get_encrypt_key should succeed");
    assert!(!key.is_empty());

    let params = enc.get_params().expect("get_params should return Some");
    assert!(params.get("zcid").and_then(Value::as_str).is_some());
    assert!(params.get("zcid_ext").and_then(Value::as_str).is_some());
    assert_eq!(params.get("enc_ver").and_then(Value::as_str), Some("v2"));
}

#[test]
fn get_sign_key_produces_deterministic_output() {
    let mut params = Map::new();
    params.insert("imei".to_string(), json!("abc-123"));
    params.insert("ts".to_string(), json!(1700000000));
    params.insert("type".to_string(), json!(30));

    let a = get_sign_key("30", &params);
    let b = get_sign_key("30", &params);
    let c = get_sign_key("30", &params);
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a.len(), 32);

    // Different api_type yields different key
    let d = get_sign_key("31", &params);
    assert_ne!(a, d);
}

#[test]
fn encode_decode_aes_round_trip_with_login_like_data() {
    // Simulate login-like payload encrypted with a base64 AES key
    let key_bytes = [0x42u8; 16]; // AES-128
    let key_b64 = BASE64.encode(key_bytes);
    let login_payload = json!({
        "imei": "device-uuid-xyz",
        "phone": "84000000000",
        "ts": 1720000000000i64,
        "params": { "enc_ver": "v2" }
    })
    .to_string();

    let encrypted = encode_aes(&key_b64, &login_payload).expect("encryption should succeed");
    assert_ne!(encrypted, login_payload);
    assert!(BASE64.decode(&encrypted).is_ok(), "ciphertext should be base64");

    let decrypted = decode_aes(&key_b64, &encrypted).expect("decryption should succeed");
    assert_eq!(decrypted, login_payload);

    let parsed: Value = serde_json::from_str(&decrypted).expect("should parse back to JSON");
    assert_eq!(parsed["imei"], "device-uuid-xyz");
}
