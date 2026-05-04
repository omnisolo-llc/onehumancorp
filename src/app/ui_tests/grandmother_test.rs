use slint::ComponentHandle;
use slint::SharedString;

#[test]
fn test_login_plain_language() {
    let ui = crate::app::Login::new().unwrap();
    assert_eq!(ui.get_title(), "OneHumanCorp - Login");
}

#[test]
fn test_login_error_message_is_empty() {
    let ui = crate::app::Login::new().unwrap();
    assert_eq!(ui.get_error_message(), "");
}

#[test]
fn test_login_verification_message_is_empty() {
    let ui = crate::app::Login::new().unwrap();
    assert_eq!(ui.get_verification_message(), "");
}

#[test]
fn test_login_show_verification_is_false() {
    let ui = crate::app::Login::new().unwrap();
    assert_eq!(ui.get_show_verification(), false);
}

#[test]
fn test_login_is_sign_up_is_false() {
    let ui = crate::app::Login::new().unwrap();
    assert_eq!(ui.get_is_sign_up(), false);
}
