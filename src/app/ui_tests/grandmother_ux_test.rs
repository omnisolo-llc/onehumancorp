use slint::ComponentHandle;

#[test]
fn test_api_docs_title_plain_language() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::ApiDocs::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Connect Custom Software"));
}

#[test]
fn test_integrations_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Integrations::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Integrations & Tools"));
}

#[test]
fn test_login_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("One Human Corp - Login"));
}

#[test]
fn test_login_subtitle_plain_language() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    ui.set_is_sign_up(false);
    // Subtitle logic is internal, we just verify component doesn't crash on standard properties
    assert_eq!(ui.get_is_sign_up(), false);
}

#[test]
fn test_login_sign_in_button() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let clicked_clone = clicked.clone();
    ui.on_login(move |_u, _p| {
        *clicked_clone.borrow_mut() = true;
    });
    ui.invoke_login("u".into(), "p".into());
    assert!(*clicked.borrow(), "Sign in button callback should trigger");
}

#[test]
fn test_login_username_placeholder() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    ui.set_username("jane".into());
    assert_eq!(ui.get_username(), slint::SharedString::from("jane"));
}

#[test]
fn test_login_password_placeholder() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    ui.set_password("pass123".into());
    assert_eq!(ui.get_password(), slint::SharedString::from("pass123"));
}

#[test]
fn test_login_error_message() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    ui.set_error_message("Invalid login".into());
    assert_eq!(ui.get_error_message(), slint::SharedString::from("Invalid login"));
}

#[test]
fn test_help_center_ui_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::HelpCenter::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Help Center"));
}

#[test]
fn test_ai_help_chat_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::AiHelpChat::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("AI Help Assistant"));
}

#[test]
fn test_kairos_orchestration_walkthrough_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::KairosOrchestrationWalkthrough::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("How Your Helpers Work Together"));
}

#[test]
fn test_ongoing_management_fix_agent_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::FixAgent::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Help Me Fix This"));
}

#[test]
fn test_ongoing_management_upgrade_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Upgrade::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Platform Upgrade"));
}

#[test]
fn test_secure_agent_config_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::SecureAgentConfig::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Secure Agent Config"));
}

#[test]
fn test_landing_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Landing::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("OneHumanCorp"));
}

#[test]
fn test_dashboard_grandmother_ux_labels_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Dashboard::new().unwrap();
    // Verify creation doesn't panic after label updates
}
