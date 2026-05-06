use crate::app;

fn create() -> app::ApiDocs {
    crate::ui_tests::init();
    app::ApiDocs::new().unwrap()
}

// --- Specialized / Flow Tests ---

#[test]
fn docs_flow_config_sync() {
    let ui = create();
    ui.set_api_key("sk_test_123".into());
    ui.set_endpoint_url("http://localhost:8080".into());
    assert_eq!(ui.get_api_key(), "sk_test_123");
    assert_eq!(ui.get_endpoint_url(), "http://localhost:8080");
}

#[test]
fn docs_xss_key() {
    let ui = create();
    let xss = "<script>alert('api_key')</script>";
    ui.set_api_key(xss.into());
    assert_eq!(ui.get_api_key(), xss);
}

#[test]
fn docs_long_endpoint() {
    let ui = create();
    let long = "https://".to_string() + &"a".repeat(1000) + ".com";
    ui.set_endpoint_url(long.clone().into());
    assert_eq!(ui.get_endpoint_url(), long);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_api_key() {
    let ui = create();
    ui.set_api_key("sk_live_555".into());
    assert_eq!(ui.get_api_key(), "sk_live_555");
    ui.set_api_key("sk_sandbox_xyz".into());
    assert_eq!(ui.get_api_key(), "sk_sandbox_xyz");
    ui.set_api_key("k11".into());
    assert_eq!(ui.get_api_key(), "k11");
}

#[test]
fn create_verify_endpoint_url() {
    let ui = create();
    ui.set_endpoint_url("https://api.v2.ohc.io".into());
    assert_eq!(ui.get_endpoint_url(), "https://api.v2.ohc.io");
    ui.set_endpoint_url("e21".into());
    assert_eq!(ui.get_endpoint_url(), "e21");
    ui.set_endpoint_url("e22".into());
    assert_eq!(ui.get_endpoint_url(), "e22");
}

#[test]
fn test_api_docs_is_advanced_toggle() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    let ui = create();
    assert_eq!(ui.get_is_advanced(), false); // Should be false by default
    ui.set_is_advanced(true);
    assert_eq!(ui.get_is_advanced(), true);
}
