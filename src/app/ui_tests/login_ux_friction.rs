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
    assert_eq!(ui.get_sso_button_text(), "Continue with Google/Apple");
}
