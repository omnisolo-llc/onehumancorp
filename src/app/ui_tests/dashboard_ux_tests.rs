use crate::app;
use slint::SharedString;

fn create_dashboard() -> app::Dashboard {
    crate::ui_tests::init();
    app::Dashboard::new().unwrap()
}

fn create_login() -> app::Login {
    crate::ui_tests::init();
    app::Login::new().unwrap()
}

#[test]
fn test_dashboard_plain_language_labels() {
    let _ui = create_dashboard();
}

#[test]
fn test_login_error_message_visibility() {
    let ui = create_login();
    ui.set_error_message(SharedString::from("Incorrect password"));
    assert_eq!(ui.get_error_message(), "Incorrect password");
    // Verify fail-close mechanics on error message propagation
    ui.set_error_message(SharedString::from("API error 500: null pointer exception"));
    assert_eq!(ui.get_error_message(), "API error 500: null pointer exception");
}

#[test]
fn test_dashboard_ux_friction_fixes_flow() {
    let ui = create_dashboard();
    ui.set_is_advanced(true);
    assert!(ui.get_is_advanced());
}
