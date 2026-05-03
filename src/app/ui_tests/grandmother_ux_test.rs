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
    assert_eq!(ui.get_loading(), false);
    assert_eq!(ui.get_show_verification(), false);
}

#[test]
fn test_login_toggle_sign_up() {
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_is_sign_up(), false);
    ui.set_is_sign_up(true);
    assert_eq!(ui.get_is_sign_up(), true);
}

#[test]
fn test_login_username_property() {
    let ui = app::Login::new().unwrap();
    ui.set_username(slint::SharedString::from("testuser"));
    assert_eq!(ui.get_username(), slint::SharedString::from("testuser"));
}

#[test]
fn test_login_password_property() {
    let ui = app::Login::new().unwrap();
    ui.set_password(slint::SharedString::from("password123"));
    assert_eq!(ui.get_password(), slint::SharedString::from("password123"));
}

#[test]
fn test_login_responsive_375() {
    let ui = app::Login::new().unwrap();
    let window = ui.window();
    window.set_size(slint::PhysicalSize::new(375, 812));
    assert_eq!(window.size().width, 375);
}

#[test]
fn test_login_responsive_1440() {
    let ui = app::Login::new().unwrap();
    let window = ui.window();
    window.set_size(slint::PhysicalSize::new(1440, 900));
    assert_eq!(window.size().width, 1440);
}
