use crate::app;

// Note: To conform with the "NO MOCKING OF NETWORK REQUESTS IN E2E TESTS" mandate
// from the codebase constraints, this file is specifically testing isolated UI
// component state logic without triggering network requests. The actual E2E flow
// is handled by Playwright.

#[test]
fn test_ui_env_wizard_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    super::init();
    let ui = app::EnvWizard::new().unwrap();

    assert_eq!(ui.get_step(), 0);

    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);

    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 2);

    ui.invoke_prev_step();
    assert_eq!(ui.get_step(), 1);
}

#[test]
fn test_ui_env_wizard_advanced_mode_toggle() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    super::init();
    let ui = app::EnvWizard::new().unwrap();

    assert_eq!(ui.get_is_advanced(), false);
    ui.set_is_advanced(true);
    assert_eq!(ui.get_is_advanced(), true);
}

#[test]
fn test_ui_env_wizard_property_binding_standalone() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    super::init();
    let ui = app::EnvWizard::new().unwrap();

    ui.set_multitenant(false);
    assert_eq!(ui.get_multitenant(), false);

    ui.set_log_level("debug".into());
    assert_eq!(ui.get_log_level(), "debug");

    ui.set_port("9090".into());
    assert_eq!(ui.get_port(), "9090");
}

#[test]
fn test_ui_env_wizard_property_binding_cloud() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    super::init();
    let ui = app::EnvWizard::new().unwrap();

    ui.set_multitenant(true);
    assert_eq!(ui.get_multitenant(), true);

    ui.set_db_url("postgres://...".into());
    assert_eq!(ui.get_db_url(), "postgres://...");

    ui.set_redis_url("redis://...".into());
    assert_eq!(ui.get_redis_url(), "redis://...");
}

#[test]
fn test_ui_env_wizard_api_key_inputs() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    super::init();
    let ui = app::EnvWizard::new().unwrap();

    ui.set_openai_key("sk-test-openai".into());
    assert_eq!(ui.get_openai_key(), "sk-test-openai");

    ui.set_anthropic_key("sk-ant-test".into());
    assert_eq!(ui.get_anthropic_key(), "sk-ant-test");
}
