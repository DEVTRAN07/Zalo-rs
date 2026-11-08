//! Send report to Zalo.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::get_user_info::service_url;
use crate::context::ContextSession;
use crate::errors::{Result, ZaloApiError, ZaloError};
use crate::factory::FactoryUtils;
use crate::models::r#enum::ThreadType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(i32)]
pub enum ReportReason {
    Sensitive = 1,
    Annoy = 2,
    Fraud = 3,
    Other = 0,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendReportOptions {
    pub reason: ReportReason,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendReportResponse {
    #[serde(rename = "reportId", default)]
    pub report_id: String,
}

/// Send report to Zalo.
pub async fn send_report(
    ctx: &ContextSession,
    service_map: &Value,
    options: SendReportOptions,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<SendReportResponse> {
    let (path, svc, params) = match thread_type {
        ThreadType::User => {
            let mut p = serde_json::json!({
                "idTo": thread_id,
                "objId": "person.profile",
                "reason": (options.reason as i32).to_string(),
            });
            if matches!(options.reason, ReportReason::Other) {
                if let Some(c) = &options.content {
                    p["content"] = Value::String(c.clone());
                }
            }
            ("/api/report/abuse-v2", "profile", p)
        }
        ThreadType::Group => {
            let content = if matches!(options.reason, ReportReason::Other) {
                options.content.clone().unwrap_or_default()
            } else {
                String::new()
            };
            let p = serde_json::json!({
                "uidTo": thread_id,
                "type": 14,
                "reason": options.reason as i32,
                "content": content,
                "imei": ctx.imei,
            });
            ("/api/social/profile/reportabuse", "profile", p)
        }
    };

    let base_url = service_url(service_map, svc, path)?;
    let utils = FactoryUtils::new(ctx);
    let url = utils.make_url(&base_url, &[], true)?;

    let encrypted = utils
        .encode_aes(&params.to_string())
        .ok_or_else(|| ZaloError::Api(ZaloApiError::new("Failed to encrypt params")))?;

    let res = utils.request_post(&url, &[("params", encrypted)]).await?;
    utils.resolve(res).await
}
