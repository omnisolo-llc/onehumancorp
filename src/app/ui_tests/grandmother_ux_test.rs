use crate::app;
use crate::ui_tests::init;
use slint::ComponentHandle;

#[test]
fn test_login_renders_and_responds() {
    init();
    let ui = app::Login::new().unwrap();

    // Simulate setting fields
    ui.set_username("test@test.com".into());
    ui.set_password("pass123".into());

    assert_eq!(ui.get_username(), "test@test.com");
    assert_eq!(ui.get_password(), "pass123");
}

#[test]
fn test_dashboard_renders() {
    init();
    let ui = app::Dashboard::new().unwrap();
    assert!(!ui.get_loading());
}

#[test]
fn test_agents_renders() {
    init();
    let ui = app::Agents::new().unwrap();
    assert!(!ui.get_show_upgrade_prompt());
}

#[test]
fn test_unified_inbox_renders() {
    init();
    let ui = app::UnifiedInbox::new().unwrap();
    assert_eq!(ui.get_search_query(), "");
}

#[test]
fn test_walkthrough_renders() {
    init();
    let ui = app::Walkthrough::new().unwrap();
    assert_eq!(ui.get_current_step(), 0);
}
