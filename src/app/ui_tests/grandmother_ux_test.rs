use crate::*
use slint::ComponentHandle;

#[test]
fn test_api_docs_title_plain_language() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::ApiDocs::new().unwrap();
    assert_eq!(ui.get_title(), slint::SharedString::from("Connect Custom Software"));
}

#[test]
fn test_integrations_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Integrations::new().unwrap();
    assert_eq!(ui.get_title(), slint::SharedString::from("Integrations & Tools"));
}

#[test]
fn test_login_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_title(), slint::SharedString::from("One Human Corp - Login"));
}

#[test]
fn test_e2e_login_flow_app_settings() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    let settings_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let settings_opened_clone = settings_opened.clone();
    ui.on_open_settings(move || {
        *settings_opened_clone.borrow_mut() = true;
    });
    ui.invoke_open_settings();
    assert!(*settings_opened.borrow(), "Settings should open from Login");
}

#[test]
fn test_e2e_login_flow_sign_up_toggle() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_is_sign_up(), false);
    ui.set_is_sign_up(true);
    assert_eq!(ui.get_is_sign_up(), true);
}

#[test]
fn test_e2e_login_flow_oauth() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    let oauth_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let oauth_called_clone = oauth_called.clone();
    ui.on_oauth_login(move |provider| {
        assert_eq!(provider, "SSO");
        *oauth_called_clone.borrow_mut() = true;
    });
    ui.invoke_oauth_login("SSO".into());
    assert!(*oauth_called.borrow(), "OAuth login should trigger");
}

#[test]
fn test_e2e_login_flow_error_message() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_error_message(), slint::SharedString::from(""));
    ui.set_error_message("Invalid credentials".into());
    assert_eq!(ui.get_error_message(), slint::SharedString::from("Invalid credentials"));
}

#[test]
fn test_e2e_login_flow_verification_message() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.set_show_verification(true);
    assert_eq!(ui.get_show_verification(), true);
    let resend_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let resend_called_clone = resend_called.clone();
    ui.on_resend_verification(move |user| {
        assert_eq!(user, "testuser");
        *resend_called_clone.borrow_mut() = true;
    });
    ui.invoke_resend_verification("testuser".into());
    assert!(*resend_called.borrow(), "Resend verification should trigger");
}

#[test]
fn test_help_center_ui_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::HelpCenter::new().unwrap();
    assert_eq!(ui.get_title(), slint::SharedString::from("Help Center"));
}

#[test]
fn test_ai_help_chat_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::AiHelpChat::new().unwrap();
    assert_eq!(ui.get_title(), slint::SharedString::from("AI Help Assistant"));
}
