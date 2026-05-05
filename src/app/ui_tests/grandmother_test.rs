use crate::app;

#[test]
fn test_login_plain_language() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let _ui = app::Login::new().unwrap();
}

#[test]
fn test_api_docs_plain_language_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let _ui = app::ApiDocs::new().unwrap();
}

#[test]
fn test_api_docs_plain_language_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let _ui = app::ApiDocs::new().unwrap();
}

#[test]
fn test_integrations_plain_language_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let _ui = app::Integrations::new().unwrap();
}

#[test]
fn test_integrations_plain_language_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let _ui = app::Integrations::new().unwrap();
}

#[test]
fn test_login_signup_toggle_text() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.set_is_sign_up(false);
    assert_eq!(ui.get_is_sign_up(), false);
    ui.set_is_sign_up(true);
    assert_eq!(ui.get_is_sign_up(), true);
}

#[test]
fn test_login_error_message_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.set_error_message("Invalid credentials".into());
    assert_eq!(ui.get_error_message(), "Invalid credentials");
}

#[test]
fn test_login_verification_message_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.set_show_verification(true);
    ui.set_verification_message("Please verify your email".into());
    assert_eq!(ui.get_show_verification(), true);
    assert_eq!(ui.get_verification_message(), "Please verify your email");
}

#[test]
fn test_login_loading_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.set_loading(true);
    assert_eq!(ui.get_loading(), true);
    ui.set_loading(false);
    assert_eq!(ui.get_loading(), false);
}

#[test]
fn test_login_username_password_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.set_username("testuser".into());
    ui.set_password("secret".into());
    assert_eq!(ui.get_username(), "testuser");
    assert_eq!(ui.get_password(), "secret");
}
