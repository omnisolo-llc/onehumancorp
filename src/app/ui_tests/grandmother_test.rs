use slint::ComponentHandle;
use slint::SharedString;

#[test]
fn test_login_plain_language() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_title(), "One Human Corp - Login");
}

#[test]
fn test_api_docs_plain_language_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::ConnectApps::new().unwrap();
    assert_eq!(ui.get_title(), "Connect Custom Software");
}

#[test]
fn test_api_docs_plain_language_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::ConnectApps::new().unwrap();
    // Assuming we can instantiate it
}

#[test]
fn test_integrations_plain_language_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Integrations::new().unwrap();
    assert_eq!(ui.get_title(), "Integrations & Tools");
}

#[test]
fn test_integrations_plain_language_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Integrations::new().unwrap();
    // Assuming we can instantiate it
}
