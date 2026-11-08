use serde_json::json;
use zalo_rs::api::url::service_url;
use zalo_rs::errors::ZaloError;

#[test]
fn service_url_constructs_correct_url() {
    let map = json!({
        "chat": ["https://chat.zalo.me"],
        "profile": ["https://profile.zalo.me/"]
    });
    let url = service_url(&map, "chat", "/api/login").expect("should resolve");
    assert_eq!(url, "https://chat.zalo.me/api/login");

    let url2 = service_url(&map, "profile", "api/me").expect("should resolve");
    assert_eq!(url2, "https://profile.zalo.me/api/me");
}

#[test]
fn service_url_missing_service_returns_error() {
    let map = json!({ "chat": ["https://chat.zalo.me"] });
    let err = service_url(&map, "missing", "/path").unwrap_err();
    match err {
        ZaloError::Api(e) => {
            assert!(e.message.contains("missing"));
            assert!(e.message.contains("zpw_service_map"));
        }
        other => panic!("expected ZaloError::Api, got {other:?}"),
    }
}

#[test]
fn service_url_handles_leading_slash_variants() {
    let map = json!({ "svc": ["https://example.com/base/"] });
    let with = service_url(&map, "svc", "/foo/bar").unwrap();
    let without = service_url(&map, "svc", "foo/bar").unwrap();
    assert_eq!(with, "https://example.com/base/foo/bar");
    assert_eq!(without, "https://example.com/base/foo/bar");
}
