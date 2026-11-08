//! Send link.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::api::parse_link;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendLinkOptions {
    pub msg: Option<String>,
    pub link: String,
    pub ttl: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendLinkResponse {
    #[serde(rename = "msgId", default)]
    pub msg_id: Value,
}

/// Send link.
pub async fn send_link(
    ctx: &ContextSession,
    service_map: &Value,
    options: SendLinkOptions,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<SendLinkResponse> {
    let parsed = parse_link::parse_link(ctx, service_map, &options.link).await?;
    let data = parsed.data;

    let msg = if let Some(m) = options.msg.as_ref().filter(|s| !s.trim().is_empty()) {
        if m.contains(&options.link) {
            m.clone()
        } else {
            format!("{} {}", m, options.link)
        }
    } else {
        options.link.clone()
    };

    let href = data.get("href").cloned().unwrap_or(Value::Null);
    let src = data.get("src").cloned().unwrap_or(Value::Null);
    let title = data.get("title").cloned().unwrap_or(Value::Null);
    let desc = data.get("desc").cloned().unwrap_or(Value::Null);
    let thumb = data.get("thumb").cloned().unwrap_or(Value::Null);
    let media = data.get("media").cloned().unwrap_or(Value::Null);

    let mut params = serde_json::json!({
        "msg": msg,
        "href": href,
        "src": src,
        "title": title,
        "desc": desc,
        "thumb": thumb,
        "type": 2,
        "media": serde_json::to_string(&media).unwrap_or_else(|_| "null".to_string()),
        "ttl": options.ttl.unwrap_or(0),
        "clientId": chrono::Utc::now().timestamp_millis(),
    });

    let (path, base_svc) = match thread_type {
        ThreadType::Group => ("/api/group/sendlink", "group"),
        ThreadType::User => ("/api/message/link", "chat"),
    };
    let base_url = service_url(service_map, base_svc, path)?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[("nretry", "0".to_string())], true)?;

    match thread_type {
        ThreadType::Group => {
            params["grid"] = Value::String(thread_id.to_string());
            params["imei"] = Value::String(ctx.imei.clone());
        }
        ThreadType::User => {
            params["toId"] = Value::String(thread_id.to_string());
            params["mentionInfo"] = Value::String(String::new());
        }
    }

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
