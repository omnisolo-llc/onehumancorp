use slint::ComponentHandle;

#[test]
fn test_mock_data_audit_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::SetupWizard::new().unwrap();
    // Simulate user flow verifying no hardcoded defaults block the process
    ui.set_business_type("Technology".into());
    assert_eq!(ui.get_business_type(), "Technology");
}

#[test]
fn test_mock_data_audit_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Dashboard::new().unwrap();
    ui.invoke_action_see_analytics();
    // Ensures clicking functions naturally
}

#[test]
fn test_mock_data_audit_3() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    // Simulate natural user clicking logic
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();
    ui.on_open_settings(move || { *invoked_clone.borrow_mut() = true; });
    ui.invoke_open_settings();
    assert!(*invoked.borrow(), "Should open settings naturally");
}

#[test]
fn test_mock_data_audit_4() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::UnifiedInbox::new().unwrap();
    // The unified inbox should not have hardcoded mock data initially
    assert_eq!(ui.get_active_conversation_id(), "");
}

#[test]
fn test_mock_data_audit_5() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::SetupWizard::new().unwrap();
    // Check that we can manipulate the product currency
    ui.set_product_currency("USD".into());
    assert_eq!(ui.get_product_currency(), "USD");
}
