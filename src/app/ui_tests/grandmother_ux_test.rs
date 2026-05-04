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
fn test_e2e_login_open_settings() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    let settings_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let settings_opened_clone = settings_opened.clone();
    ui.on_open_settings(move || {
        *settings_opened_clone.borrow_mut() = true;
    });
    ui.invoke_open_settings();
    assert!(*settings_opened.borrow(), "Settings should open from Login via icon click");
}

#[test]
fn test_e2e_login_toggle_signup() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.set_is_sign_up(false);
    assert_eq!(ui.get_is_sign_up(), false);
    ui.set_is_sign_up(true);
    assert_eq!(ui.get_is_sign_up(), true);
}

#[test]
fn test_e2e_login_trigger_login() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    let login_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_triggered_clone = login_triggered.clone();
    ui.on_login(move |username, password| {
        assert_eq!(username, "testuser");
        assert_eq!(password, "testpass");
        *login_triggered_clone.borrow_mut() = true;
    });
    ui.invoke_login("testuser".into(), "testpass".into());
    assert!(*login_triggered.borrow(), "Login callback should trigger correctly");
}

#[test]
fn test_e2e_login_oauth() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    let oauth_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let oauth_triggered_clone = oauth_triggered.clone();
    ui.on_oauth_login(move |provider| {
        assert_eq!(provider, "SSO");
        *oauth_triggered_clone.borrow_mut() = true;
    });
    ui.invoke_oauth_login("SSO".into());
    assert!(*oauth_triggered.borrow(), "OAuth login should trigger correctly");
}

#[test]
fn test_e2e_login_show_verification() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    ui.set_show_verification(false);
    assert_eq!(ui.get_show_verification(), false);
    ui.set_show_verification(true);
    assert_eq!(ui.get_show_verification(), true);
    ui.set_verification_message("Check email".into());
    assert_eq!(ui.get_verification_message(), "Check email");
}
