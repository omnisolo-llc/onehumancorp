use crate::app;
use slint::{ComponentHandle, Model};

fn create() -> app::Login {
    crate::ui_tests::init();
    app::Login::new().unwrap()
}

#[test]
fn test_login_submit_button_text_signin_loading() {
    let ui = create();
    ui.set_is_sign_up(false);
    ui.set_loading(true);
    assert_eq!(ui.get_submit_button_text(), "Signing in...");
}

#[test]
fn test_login_submit_button_text_signup_loading() {
    let ui = create();
    ui.set_is_sign_up(true);
    ui.set_loading(true);
    assert_eq!(ui.get_submit_button_text(), "Creating account...");
}

#[test]
fn test_login_sso_button_text_loading() {
    let ui = create();
    ui.set_loading(true);
    assert_eq!(ui.get_sso_button_text(), "Connecting...");
}

#[test]
fn test_login_submit_button_text_signin_not_loading() {
    let ui = create();
    ui.set_is_sign_up(false);
    ui.set_loading(false);
    assert_eq!(ui.get_submit_button_text(), "Sign In");
}

#[test]
fn test_login_sso_button_text_not_loading() {
    let ui = create();
    ui.set_loading(false);
    assert_eq!(ui.get_sso_button_text(), "Use Google or Apple");
}

#[test]
fn test_login_echo_plain_language_google() {
    let ui = create();
    ui.set_loading(false);
    assert_eq!(ui.get_sso_button_text(), "Use Google or Apple");
}

#[test]
fn test_login_echo_plain_language_settings() {
    let ui = create();
    assert_eq!(ui.get_settings_button_text(), "⚙ Fix App Issues");
}

#[test]
fn test_login_echo_plain_language_toggle_signup() {
    let ui = create();
    ui.set_is_sign_up(false);
    assert_eq!(ui.get_toggle_button_text(), "New here? Create an account");
}

#[test]
fn test_login_echo_plain_language_toggle_signin() {
    let ui = create();
    ui.set_is_sign_up(true);
    assert_eq!(ui.get_toggle_button_text(), "Have an account? Sign In");
}

#[test]
fn test_login_echo_business_setup_click_sim() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();
    ui.on_start_setup_wizard(move || {
        *invoked_clone.borrow_mut() = true;
    });
    ui.invoke_start_setup_wizard();
    assert!(*invoked.borrow(), "Start My Business button logic must work");
}
