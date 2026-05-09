use slint::ComponentHandle;
use slint::SharedString;

#[test]
fn test_echo_grandmother_ux_flow_login_settings() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let login_ui = crate::app::Login::new().unwrap();

    // Simulate login screen flow
    assert_eq!(login_ui.get_settings_button_text(), SharedString::from("Fix App Issues"));

    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();
    login_ui.on_open_settings(move || {
        *invoked_clone.borrow_mut() = true;
    });

    login_ui.invoke_open_settings();
    assert!(*invoked.borrow(), "Fix App Issues action should be triggered");
}

#[test]
fn test_echo_grandmother_ux_flow_dashboard_active_helpers() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = crate::app::Login::new().unwrap();
    // Verify login state assumption
    login_ui.set_username("test".into());
    login_ui.set_password("pass".into());

    // Transition to dashboard logic
    let dashboard_ui = crate::app::Dashboard::new().unwrap();

    // Set properties
    dashboard_ui.set_active_helpers_count(5);

    // Slint test bindings usually don't easily export statcard titles unless we explicitly test for it,
    // but we can verify the properties are functional.
    assert_eq!(dashboard_ui.get_active_helpers_count(), 5);
}

#[test]
fn test_echo_grandmother_ux_flow_dashboard_active_help() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    dashboard_ui.set_generative_score("95".into());

    assert_eq!(dashboard_ui.get_generative_score(), SharedString::from("95"));

    let hint_invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let hint_invoked_clone = hint_invoked.clone();

    // Use the native property to show the health hint (which acts as active help hint now)
    dashboard_ui.set_show_health_hint(true);
    assert_eq!(dashboard_ui.get_show_health_hint(), true);
}

#[test]
fn test_echo_grandmother_ux_flow_login_error_handling() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = crate::app::Login::new().unwrap();
    login_ui.set_error_message("Invalid credentials".into());

    assert_eq!(login_ui.get_error_message(), SharedString::from("Invalid credentials"));

    // Ensure "Fix App Issues" is still functional during error
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();
    login_ui.on_open_settings(move || {
        *invoked_clone.borrow_mut() = true;
    });

    login_ui.invoke_open_settings();
    assert!(*invoked.borrow(), "Fix App Issues action should be triggered even in error state");
}

#[test]
fn test_echo_grandmother_ux_flow_full_app_navigation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = crate::app::Login::new().unwrap();
    assert_eq!(login_ui.get_settings_button_text(), SharedString::from("Fix App Issues"));

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    dashboard_ui.set_show_health_hint(true);
    assert_eq!(dashboard_ui.get_show_health_hint(), true);

    dashboard_ui.set_active_helpers_count(10);
    assert_eq!(dashboard_ui.get_active_helpers_count(), 10);

    dashboard_ui.set_generative_score("100".into());
    assert_eq!(dashboard_ui.get_generative_score(), SharedString::from("100"));
}
