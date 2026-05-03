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
