use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::models::r#enum::ThreadType;

pub type SendReportResponse = Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ReportOptions {
    pub reason: Option<String>,
    #[serde(rename = "content")]
    pub content: Option<String>,
}

/// Send report.
pub async fn send_report(
    client: &ApiClient,
    options: ReportOptions,
    thread_id: &str,
    thread_type: ThreadType,
) -> Result<SendReportResponse> {
    let path = match thread_type {
        ThreadType::Group => "/api/group/reportabuse-v2",
        ThreadType::User => "/api/profile/abuse-v2",
    };
    let base_url = client.service_url("profile", path)?;
    let url = client.make_url(&base_url, &[], true)?;

    let obj_id = if matches!(thread_type, ThreadType::Group) { "idTo" } else { "uIdTo" };
    let mut params = serde_json::json!({
        obj_id: thread_id,
        "objId": thread_id,
        "reason": options.reason.unwrap_or_else(|| "1".to_string()),
        "content": options.content.unwrap_or_default(),
        "imei": client.ctx().imei,
    });
    if matches!(thread_type, ThreadType::Group) {
        params["type"] = Value::from(1);
    }
    client.post_encrypted(&url, &params).await
}
