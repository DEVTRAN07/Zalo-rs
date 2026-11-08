//! Example: login with credentials and fetch user info.
//!
//! Usage:
//!   cargo run --example login
//!
//! Set environment variables or edit the credentials below:
//!   ZALO_IMEI, ZALO_USER_AGENT, ZALO_COOKIE_JSON

use zalo_client::{Credentials, ZaloClient, ZaloCookie};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example credentials — replace with real values extracted from browser.
    // Cookie can be exported as a JSON array of {name, value, domain, ...}.
    let imei = std::env::var("ZALO_IMEI").unwrap_or_else(|_| "your-imei-here".into());
    let user_agent = std::env::var("ZALO_USER_AGENT")
        .unwrap_or_else(|_| "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)".into());
    let cookie_json = std::env::var("ZALO_COOKIE_JSON").unwrap_or_else(|_| "[]".into());
    let cookies: Vec<ZaloCookie> = serde_json::from_str(&cookie_json).unwrap_or_default();

    if cookies.is_empty() || imei == "your-imei-here" {
        eprintln!("Set ZALO_IMEI, ZALO_USER_AGENT, and ZALO_COOKIE_JSON to run this example.");
        eprintln!("ZALO_COOKIE_JSON should be a JSON array of cookie objects.");
        return Ok(());
    }

    let credentials = Credentials {
        imei,
        cookie: cookies,
        user_agent,
        language: Some("vi".into()),
    };

    let client = ZaloClient::new(Default::default());
    let api = client.login(credentials).await?;

    println!("Logged in as uid={}", api.uid());

    // Fetch own user info
    let uid = api.uid().to_string();
    match api.get_user_info(&[&uid]).await {
        Ok(info) => println!("User info: {}", serde_json::to_string_pretty(&info)?),
        Err(e) => eprintln!("get_user_info error: {}", e),
    }

    // Keep alive
    let _ = api.keep_alive().await;

    // WebSocket listener example (uncomment to use)
    // let mut listener = api.listener();
    // let mut rx = listener.start()?;
    // while let Some(event) = rx.recv().await {
    //     println!("Event: {:?}", event);
    // }

    Ok(())
}
