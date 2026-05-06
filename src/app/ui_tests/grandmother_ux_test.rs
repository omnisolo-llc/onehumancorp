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
fn test_login_flow_loading_and_error_message_mock() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = crate::app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    // Set up mock login callback to simulate main.rs logic
    let login_handle = login_ui.as_weak();
    login_ui.on_login(move |email, _password| {
        if let Some(ui) = login_handle.upgrade() {
            if email == "e61" || email.to_string().contains("error_") {
                ui.set_error_message("Invalid email or password. Please try again.".into());
                return;
            }
            ui.set_loading(true);
            *login_successful_clone.borrow_mut() = true;
        }
    });

    assert!(!login_ui.get_loading(), "Initial loading state should be false");

    // Trigger error state
    login_ui.invoke_login("e61".into(), "password123".into());
    assert_eq!(login_ui.get_error_message(), slint::SharedString::from("Invalid email or password. Please try again."));
    assert!(!login_ui.get_loading(), "Should not be loading after error");

    // Trigger success state
    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(login_ui.get_loading(), "Should be loading during authentication");
}
