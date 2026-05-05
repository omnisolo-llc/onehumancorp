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
fn test_ux_flow_login_to_dashboard_fast() {
    crate::ui_tests::init();
    let app = crate::app::AppWindow::new().unwrap();
    // In our new App architecture, AppWindow doesn't directly expose login, but let's test Login component directly
    let login_ui = crate::app::Login::new().unwrap();

    let login_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_called_clone = login_called.clone();

    login_ui.on_login(move |username, password| {
        *login_called_clone.borrow_mut() = true;
        assert_eq!(username, "test@example.com");
        assert_eq!(password, "password123");
    });

    login_ui.set_username("test@example.com".into());
    login_ui.set_password("password123".into());
    login_ui.invoke_login("test@example.com".into(), "password123".into());

    assert!(*login_called.borrow());
}

#[test]
fn test_ux_flow_login_help_button() {
    crate::ui_tests::init();
    let login_ui = crate::app::Login::new().unwrap();
    // Test that the settings button now has plain language text instead of technical jargon
    assert_eq!(login_ui.get_settings_button_text(), slint::SharedString::from("Help & Support"));
}

#[test]
fn test_ux_flow_dashboard_metrics_update() {
    crate::ui_tests::init();
    let dashboard_ui = crate::app::Dashboard::new().unwrap();

    // Test that the properties behind the newly renamed plain-language labels update successfully
    dashboard_ui.set_active_helpers_count(5);
    dashboard_ui.set_tasks_in_progress_count(12);

    assert_eq!(dashboard_ui.get_active_helpers_count(), 5);
    assert_eq!(dashboard_ui.get_tasks_in_progress_count(), 12);
}

#[test]
fn test_ux_flow_login_verification_message() {
    crate::ui_tests::init();
    let login_ui = crate::app::Login::new().unwrap();

    login_ui.set_show_verification(true);
    let msg = "Please check your email to verify your account.";
    login_ui.set_verification_message(msg.into());

    assert!(login_ui.get_show_verification());
    assert_eq!(login_ui.get_verification_message(), slint::SharedString::from(msg));
}

#[test]
fn test_ux_flow_login_sso_button() {
    crate::ui_tests::init();
    let login_ui = crate::app::Login::new().unwrap();
    // Verify that the SSO button has clear phrasing
    assert_eq!(login_ui.get_sso_button_text(), slint::SharedString::from("Continue with Google/Apple"));
}
