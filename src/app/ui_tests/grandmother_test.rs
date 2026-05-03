use slint::ComponentHandle;
use slint::SharedString;

#[test]
fn test_login_plain_language() {
    let ui = app::Login::new().unwrap();
    assert_eq!(ui.get_title(), "One Human Corp - Login");
}

#[test]
fn test_api_docs_plain_language_1() {
    let ui = app::ApiDocs::new().unwrap();
    assert_eq!(ui.get_title(), "Connect Custom Software");
}

#[test]
fn test_api_docs_plain_language_2() {
    let ui = app::ApiDocs::new().unwrap();
}

#[test]
fn test_integrations_plain_language_1() {
    let ui = app::Integrations::new().unwrap();
    assert_eq!(ui.get_title(), "Integrations & Tools");
}

#[test]
fn test_integrations_plain_language_2() {
    let ui = app::Integrations::new().unwrap();
}

#[test]
fn test_login_responsive_width_preferred() {
    let ui = app::Login::new().unwrap();
    let win = ui.window();
    let size = win.size();
    assert!(size.width >= 375, "Window width should be at least 375px");
}

#[test]
fn test_dashboard_responsive_width_preferred() {
    let ui = app::Dashboard::new().unwrap();
    let win = ui.window();
    let size = win.size();
    assert!(size.width >= 375, "Window width should be at least 375px");
}

#[test]
fn test_setup_wizard_responsive_width_preferred() {
    let ui = app::SetupWizard::new().unwrap();
    let win = ui.window();
    let size = win.size();
    assert!(size.width >= 375, "Window width should be at least 375px");
}

#[test]
fn test_pricing_responsive_width_preferred() {
    let ui = app::Pricing::new().unwrap();
    let win = ui.window();
    let size = win.size();
    assert!(size.width >= 375, "Window width should be at least 375px");
}

#[test]
fn test_settings_responsive_width_preferred() {
    let ui = app::Settings::new().unwrap();
    let win = ui.window();
    let size = win.size();
    assert!(size.width >= 375, "Window width should be at least 375px");
}
