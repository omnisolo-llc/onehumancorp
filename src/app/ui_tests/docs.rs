use crate::app;

fn create() -> app::ApiDocs { crate::ui_tests::init(); app::ApiDocs::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn docs_flow_config_sync() {
    let ui = create();
    ui.set_api_key("sk_test_123".into());
    ui.set_endpoint_url("http://localhost:8080".into());
    assert_eq!(ui.get_api_key(), "sk_test_123");
    assert_eq!(ui.get_endpoint_url(), "http://localhost:8080");
}

#[test] fn docs_xss_key() {
    let ui = create();
    let xss = "<script>alert('api_key')</script>";
    ui.set_api_key(xss.into());
    assert_eq!(ui.get_api_key(), xss);
}

#[test] fn docs_long_endpoint() {
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
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = create();
    assert_eq!(ui.get_is_advanced(), false); // Should be false by default
    ui.set_is_advanced(true);
    assert_eq!(ui.get_is_advanced(), true);
}

#[test]
fn test_e2e_api_docs_interactive_swagger() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = crate::app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        assert_eq!(email, "test@example.com");
        assert_eq!(password, "password123");
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "Login should be successful");

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let api_docs_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let api_docs_opened_clone = api_docs_opened.clone();

    dashboard_ui.on_open_api_docs(move || {
        *api_docs_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_open_api_docs();
    assert!(*api_docs_opened.borrow(), "Api Docs should be opened from Dashboard");

    let api_docs = crate::app::ApiDocs::new().unwrap();

    // We can't actually do a real API call since there's no backend running in this test.
    // However, the test should still successfully invoke the endpoint to verify the code path executes.
    api_docs.invoke_test_endpoint();
    assert_eq!(api_docs.get_is_testing(), false);
}
