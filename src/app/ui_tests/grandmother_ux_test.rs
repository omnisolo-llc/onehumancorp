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
fn test_login_initial_state() {
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_is_sign_up(), false);
    assert_eq!(ui.get_error_message(), slint::SharedString::from(""));
    assert_eq!(ui.get_show_verification(), false);
    assert_eq!(ui.get_verification_message(), slint::SharedString::from(""));
}

#[test]
fn test_login_set_username_password() {
    let ui = app::Login::new().unwrap();
    ui.set_username(slint::SharedString::from("test@example.com"));
    ui.set_password(slint::SharedString::from("password123"));
    assert_eq!(ui.get_username(), slint::SharedString::from("test@example.com"));
    assert_eq!(ui.get_password(), slint::SharedString::from("password123"));
}

#[test]
fn test_login_toggle_sign_up() {
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_is_sign_up(), false);
    ui.set_is_sign_up(true);
    assert_eq!(ui.get_is_sign_up(), true);
}

#[test]
fn test_login_show_error() {
    let ui = app::Login::new().unwrap();
    ui.set_error_message(slint::SharedString::from("Invalid credentials"));
    assert_eq!(ui.get_error_message(), slint::SharedString::from("Invalid credentials"));
}

#[test]
fn test_login_show_verification() {
    let ui = app::Login::new().unwrap();
    ui.set_show_verification(true);
    ui.set_verification_message(slint::SharedString::from("Verification sent!"));
    assert_eq!(ui.get_show_verification(), true);
    assert_eq!(ui.get_verification_message(), slint::SharedString::from("Verification sent!"));
}
