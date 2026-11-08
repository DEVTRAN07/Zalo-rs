//! Login endpoints: `getLoginInfo` and `getServerInfo`.

use serde_json::Value;

use crate::context::{ContextBase, ServerInfo};
use crate::crypto::{decrypt_resp, get_sign_key, ParamsEncryptor};
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::http;
use crate::zlog_error;

struct EncryptParamResult {
    params: serde_json::Map<String, Value>,
    enk: Option<String>,
}

/// Fetch login info from the Zalo web API.
pub async fn login(ctx: &ContextBase, encrypt_params: bool) -> Result<Option<Value>> {
    let enc = get_encrypt_param(ctx, encrypt_params, "getlogininfo")?;

    let mut query_params: Vec<(&str, String)> = enc
        .params
        .iter()
        .map(|(k, v)| {
            let s = match v {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                other => other.to_string(),
            };
            (k.as_str(), s)
        })
        .collect();
    query_params.push(("nretry", "0".to_string()));

    let url = http::make_url(ctx, "https://wpa.chat.zalo.me/api/login/getLoginInfo", &query_params, true)?;

    let response = http::post_form(ctx, &url, &[]).await?;
    if !response.status().is_success() {
        return Err(ZaloError::Api(ZaloApiError::new(format!(
            "Failed to fetch login info: {}",
            response.status()
        ))));
    }
    let data: Value = response.json().await?;

    if let Some(enk) = enc.enk {
        if let Some(encrypted_data) = data.get("data").and_then(|v| v.as_str()) {
            let decrypted = decrypt_resp(&enk, encrypted_data);
            return Ok(decrypted);
        }
        return Ok(None);
    }

    // Unencrypted path - return data as-is
    Ok(Some(data))
}

/// Fetch server info from the Zalo web API.
pub async fn get_server_info(ctx: &ContextBase, encrypt_params: bool) -> Result<ServerInfo> {
    let enc = get_encrypt_param(ctx, encrypt_params, "getserverinfo")?;

    let signkey = enc
        .params
        .get("signkey")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Missing signkey")))?;

    let imei = ctx.imei.as_deref().unwrap_or_default();
    let query_params = vec![
        ("imei", imei.to_string()),
        ("type", ctx.api_type.to_string()),
        ("client_version", ctx.api_version.to_string()),
        ("computer_name", "Web".to_string()),
        ("signkey", signkey.to_string()),
    ];

    let url = http::make_url(
        ctx,
        "https://wpa.chat.zalo.me/api/login/getServerInfo",
        &query_params,
        false,
    )?;

    let response = http::post_form(ctx, &url, &[]).await?;
    if !response.status().is_success() {
        return Err(ZaloError::Api(ZaloApiError::new(format!(
            "Failed to fetch server info: {}",
            response.status()
        ))));
    }
    let data: Value = response.json().await?;

    if data.get("data").is_none() || data.get("data").unwrap().is_null() {
        let msg = data
            .get("error_message")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        return Err(ZaloError::Api(ZaloApiError::new(format!(
            "Failed to fetch server info: {}",
            msg
        ))));
    }

    let server: ServerInfo = serde_json::from_value(data["data"].clone()).unwrap_or_default();
    Ok(server)
}

fn get_encrypt_param(ctx: &ContextBase, encrypt_params: bool, api_name: &str) -> Result<EncryptParamResult> {
    let imei = ctx.imei.as_deref().unwrap_or_default();
    let language = ctx.language.as_deref().unwrap_or("vi");
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let data = serde_json::json!({
        "computer_name": "Web",
        "imei": imei,
        "language": language,
        "ts": ts,
    });

    let mut params = serde_json::Map::new();
    let encrypted = _encrypt_param(ctx, &data, encrypt_params)?;

    if let Some(enc_data) = encrypted {
        if let Some(enc_params) = enc_data.encrypted_params {
            if let Value::Object(map) = enc_params {
                for (k, v) in map {
                    params.insert(k, v);
                }
            }
        }
        if let Some(ed) = enc_data.encrypted_data {
            params.insert("params".to_string(), Value::String(ed));
        }
        params.insert("type".to_string(), Value::Number(ctx.api_type.into()));
        params.insert("client_version".to_string(), Value::Number(ctx.api_version.into()));

        let signkey = if api_name == "getserverinfo" {
            let mut sk_params = serde_json::Map::new();
            sk_params.insert("imei".into(), Value::String(imei.to_string()));
            sk_params.insert("type".into(), Value::Number(ctx.api_type.into()));
            sk_params.insert("client_version".into(), Value::Number(ctx.api_version.into()));
            sk_params.insert("computer_name".into(), Value::String("Web".into()));
            get_sign_key(api_name, &sk_params)
        } else {
            get_sign_key(api_name, &params)
        };
        params.insert("signkey".to_string(), Value::String(signkey));

        Ok(EncryptParamResult {
            params,
            enk: enc_data.enk,
        })
    } else {
        if let Value::Object(map) = data {
            for (k, v) in map {
                params.insert(k, v);
            }
        }
        params.insert("type".to_string(), Value::Number(ctx.api_type.into()));
        params.insert("client_version".to_string(), Value::Number(ctx.api_version.into()));

        let signkey = if api_name == "getserverinfo" {
            let mut sk_params = serde_json::Map::new();
            sk_params.insert("imei".into(), Value::String(imei.to_string()));
            sk_params.insert("type".into(), Value::Number(ctx.api_type.into()));
            sk_params.insert("client_version".into(), Value::Number(ctx.api_version.into()));
            sk_params.insert("computer_name".into(), Value::String("Web".into()));
            get_sign_key(api_name, &sk_params)
        } else {
            get_sign_key(api_name, &params)
        };
        params.insert("signkey".to_string(), Value::String(signkey));

        Ok(EncryptParamResult { params, enk: None })
    }
}

struct EncryptedParamData {
    encrypted_data: Option<String>,
    encrypted_params: Option<Value>,
    enk: Option<String>,
}

fn _encrypt_param(
    ctx: &ContextBase,
    data: &Value,
    encrypt_params: bool,
) -> Result<Option<EncryptedParamData>> {
    if !encrypt_params {
        return Ok(None);
    }
    let imei = data.get("imei").and_then(|v| v.as_str()).unwrap_or("");
    let first_launch_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let encryptor = ParamsEncryptor::new(ctx.api_type, imei, first_launch_time).map_err(|e| {
        zlog_error!("Failed to create ParamsEncryptor: {}", e);
        ZaloError::Api(ZaloApiError::new(format!("Failed to encrypt params: {}", e)))
    })?;

    let stringified = serde_json::to_string(data)?;
    let encrypted_key = encryptor.get_encrypt_key().map_err(|e| {
        ZaloError::Api(ZaloApiError::new(format!("Failed to get encrypt key: {}", e)))
    })?;

    let encoded_data = crate::crypto::encode_aes_utf8_key(encrypted_key, &stringified, "base64", false);
    let params = encryptor.get_params();

    if params.is_none() {
        return Ok(None);
    }

    Ok(Some(EncryptedParamData {
        encrypted_data: encoded_data,
        encrypted_params: params,
        enk: Some(encrypted_key.to_string()),
    }))
}
