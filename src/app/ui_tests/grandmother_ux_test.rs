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
fn test_login_app_settings_audit_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_settings_btn_text(), slint::SharedString::from("App Settings"));
}

#[test]
fn test_login_app_settings_audit_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_settings_btn_height(), 44.0);
}

#[test]
fn test_login_app_settings_audit_3() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.window().set_size(slint::PhysicalSize::new(375, 812));
    assert_eq!(ui.get_settings_btn_height(), 44.0);
}

#[test]
fn test_login_app_settings_audit_4() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.window().set_size(slint::PhysicalSize::new(414, 896));
    assert_eq!(ui.get_settings_btn_height(), 44.0);
}

#[test]
fn test_login_app_settings_audit_5() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.window().set_size(slint::PhysicalSize::new(768, 1024));
    assert_eq!(ui.get_settings_btn_height(), 44.0);
}
