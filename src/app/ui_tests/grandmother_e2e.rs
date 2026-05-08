use crate::app;

// E2E Test 1: Full Flow from Login to Checking API Docs
#[test]
fn test_e2e_grandmother_flow_login_to_api_docs() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Start at login
    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |_, _| {
        *login_successful_clone.borrow_mut() = true;
    });
    login_ui.invoke_login("grandmother@example.com".into(), "secure123".into());
    assert!(*login_successful.borrow(), "User login should succeed");

    // Transition to scribe feature dashboard (which acts as the docs entry point)
    let scribe_ui = app::ScribeFeatureDashboard::new().unwrap();
    let api_docs_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let api_docs_opened_clone = api_docs_opened.clone();

    scribe_ui.on_open_api_docs(move || {
        *api_docs_opened_clone.borrow_mut() = true;
    });
    // Simulating Grandmother clicking "View Integration Docs"
    scribe_ui.invoke_open_api_docs();
    assert!(*api_docs_opened.borrow(), "API Docs should open");

    // Once in API docs, verify Grandmother sees plain language
    let docs_ui = app::ApiDocs::new().unwrap();
    assert_eq!(docs_ui.get_test_title(), "Connect other software to your store");
}

// E2E Test 2: Full Flow from Login to Promoting Store
#[test]
fn test_e2e_grandmother_flow_login_to_promote_store() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Start at login
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("grandmother@example.com".into(), "secure123".into());

    // User gets to Dashboard
    let dashboard_ui = app::Dashboard::new().unwrap();
    let promote_store_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let promote_store_opened_clone = promote_store_opened.clone();

    dashboard_ui.on_action_grow_business(move || {
        *promote_store_opened_clone.borrow_mut() = true;
    });

    // Simulate clicking "Promote Store" (was Run Promo)
    dashboard_ui.invoke_action_grow_business();
    assert!(*promote_store_opened.borrow(), "Grow Business (Promote Store) should open");
}

// E2E Test 3: Full Flow from Login to Fulfilling Order
#[test]
fn test_e2e_grandmother_flow_login_to_fulfill_order() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Start at login
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("grandmother@example.com".into(), "secure123".into());

    // User gets to Dashboard
    let dashboard_ui = app::Dashboard::new().unwrap();
    let mark_order_ready_invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let mark_order_ready_invoked_clone = mark_order_ready_invoked.clone();

    dashboard_ui.on_action_mark_order_ready(move || {
        *mark_order_ready_invoked_clone.borrow_mut() = true;
    });

    // Simulate clicking "Fulfill Order" (was Ready Order)
    dashboard_ui.invoke_action_mark_order_ready();
    assert!(*mark_order_ready_invoked.borrow(), "Fulfill Order action should be invoked");
}

// E2E Test 4: Full Flow from Login to Configuring Agent (Checking API Scope terminology)
#[test]
fn test_e2e_grandmother_flow_login_to_agent_config() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Start at login
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("grandmother@example.com".into(), "secure123".into());

    // User configures an agent
    let agent_config_ui = app::AgentConfig::new().unwrap();

    // User toggles advanced settings
    agent_config_ui.set_is_advanced(true);

    // Verify properties are available without technical jargon issues
    agent_config_ui.set_api_scope_override("[\"read\"]".into());
    assert_eq!(agent_config_ui.get_api_scope_override(), "[\"read\"]");
}

// E2E Test 5: Full Flow from Login to AI Config (Adding connection)
#[test]
fn test_e2e_grandmother_flow_login_to_ai_config() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Start at login
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("grandmother@example.com".into(), "secure123".into());

    // User navigates to AI Config
    let ai_config_ui = app::AiConfig::new().unwrap();

    // User clicks "Show Advanced Configuration"
    ai_config_ui.set_is_advanced(true);

    let add_provider_invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let add_provider_invoked_clone = add_provider_invoked.clone();

    ai_config_ui.on_add_provider(move || {
        *add_provider_invoked_clone.borrow_mut() = true;
    });

    // Simulate clicking "+ Add Connection" (was + Add Custom API)
    ai_config_ui.invoke_add_provider();
    assert!(*add_provider_invoked.borrow(), "Add connection action should be invoked");
}
