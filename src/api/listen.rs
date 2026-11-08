use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::ws::Listener;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListenOptions {
    #[serde(default)]
    pub retry_on_close: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListenResponse(pub Value);

pub async fn listen(
    client: &ApiClient,
    options: Option<ListenOptions>,
) -> Result<ListenResponse> {
    let _ = options; // retry_on_close handled by Listener internally

    let ws_urls = client.ws_urls().clone();
    if ws_urls.is_empty() {
        return Err(crate::errors::ZaloError::Api(
            crate::errors::ZaloApiError::new("No WebSocket URLs available")
        ));
    }

    // Create and start the listener
    let mut listener = Listener::new(client.ctx().clone(), ws_urls.clone());
    let _rx = listener.start()?;

    Ok(ListenResponse(serde_json::json!({
        "status": "listening",
        "ws_urls": ws_urls.len()
    })))
}

pub fn create_listener(client: &ApiClient, ws_urls: Vec<String>) -> Listener {
    Listener::new(client.ctx().clone(), ws_urls)
}
