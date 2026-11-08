use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateLabelsPayload {
    #[serde(rename = "labelData", default)]
    pub label_data: Vec<Value>,
    #[serde(default)]
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateLabelsResponse {
    #[serde(rename = "labelData", default)]
    pub label_data: Vec<Value>,
    #[serde(default)]
    pub version: i64,
    #[serde(rename = "lastUpdateTime", default)]
    pub last_update_time: i64,
}

/// Update labels.
pub async fn update_labels(
    client: &ApiClient,
    payload: UpdateLabelsPayload,
) -> Result<UpdateLabelsResponse> {
    let label_data_str = serde_json::to_string(&payload.label_data).unwrap_or_else(|_| "[]".to_string());
    let params = serde_json::json!({
        "labelData": label_data_str,
        "version": payload.version,
        "imei": client.ctx().imei,
    });

    let base_url = client.service_url("label", "/api/convlabel/update")?;
    let url = client.make_url(&base_url, &[], true)?;

    // Response may encode labelData as a JSON string; normalize it.
    let raw: Value = client.post_encrypted(&url, &params).await?;
    let label_data = if let Some(s) = raw.get("labelData").and_then(|v| v.as_str()) {
        serde_json::from_str(s).unwrap_or_default()
    } else {
        raw.get("labelData")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default()
    };

    Ok(UpdateLabelsResponse {
        label_data,
        version: raw.get("version").and_then(|v| v.as_i64()).unwrap_or(0),
        last_update_time: raw
            .get("lastUpdateTime")
            .and_then(|v| v.as_i64())
            .unwrap_or(0),
    })
}
