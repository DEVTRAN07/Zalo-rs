use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::api::client::ApiClient;
use crate::errors::Result;
use crate::ws::{Listener, ListenerEvent};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListenOptions {
    #[serde(default)]
    pub retry_on_close: bool,
}

#[derive(Debug)]
pub struct ListenResponse {
    pub status: String,
    pub ws_urls: usize,
    pub rx: mpsc::UnboundedReceiver<ListenerEvent>,
}

pub async fn listen(
    client: &ApiClient,
    options: Option<ListenOptions>,
) -> Result<ListenResponse> {
    let _ = options;

    let ws_urls = client.ws_urls().clone();
    if ws_urls.is_empty() {
        return Err(crate::errors::ZaloError::Api(
            crate::errors::ZaloApiError::new("No WebSocket URLs available")
        ));
    }

    let mut listener = Listener::new(client.ctx().clone(), ws_urls.clone());
    let rx = listener.start()?;

    Ok(ListenResponse {
        status: "listening".into(),
        ws_urls: ws_urls.len(),
        rx,
    })
}

pub fn create_listener(client: &ApiClient, ws_urls: Vec<String>) -> Listener {
    Listener::new(client.ctx().clone(), ws_urls)
}
