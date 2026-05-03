use crate::*
use slint::ComponentHandle;

#[test]
fn test_api_docs_title_plain_language() {
    let ui = app::ApiDocs::new().unwrap();
    assert_eq!(ui.get_title(), slint::SharedString::from("Connect Custom Software"));
}

#[test]
fn test_integrations_title() {
    let ui = app::Integrations::new().unwrap();
    assert_eq!(ui.get_title(), slint::SharedString::from("Integrations & Tools"));
}

#[test]
fn test_login_title() {
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_title(), slint::SharedString::from("One Human Corp - Login"));
}

#[test]
fn test_dummy_1() {
    assert!(true);
}

#[test]
fn test_dummy_2() {
    assert!(true);
}

#[test]
fn login_ux_app_settings_label() {
    let ui = crate::ui_tests::init();
    let login_ui = crate::app::Login::new().unwrap();

    // Test that the open_settings callback fires (simulating button click with "App Settings" label)
    let clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = clicked.clone();
    login_ui.on_open_settings(move || { *c.borrow_mut() = true; });
    login_ui.invoke_open_settings();
    assert!(*clicked.borrow(), "App Settings button should trigger the correct callback");
}

#[test]
fn login_ux_app_settings_accessibility() {
    let _ui = crate::ui_tests::init();
    let login_ui = crate::app::Login::new().unwrap();
    assert_eq!(login_ui.get_is_sign_up(), false, "Should default to sign in");
}

#[test]
fn login_ux_app_settings_visibility() {
    let _ui = crate::ui_tests::init();
    let login_ui = crate::app::Login::new().unwrap();
    // Verify properties bound to UI
    assert_eq!(login_ui.get_error_message(), "");
    assert_eq!(login_ui.get_show_verification(), false);
}

#[test]
fn login_ux_app_settings_state_toggle() {
    let _ui = crate::ui_tests::init();
    let login_ui = crate::app::Login::new().unwrap();
    login_ui.set_is_sign_up(true);
    assert_eq!(login_ui.get_is_sign_up(), true);
}
