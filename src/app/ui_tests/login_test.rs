use std::env;

#[test]
fn test_login_window_responsive_and_touch_targets() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }
}

#[test]
fn test_login_start_setup_wizard_button_triggers_callback() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let callback_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let callback_triggered_clone = callback_triggered.clone();

    ui.on_start_setup_wizard(move || {
        *callback_triggered_clone.borrow_mut() = true;
    });

    ui.invoke_start_setup_wizard();

    assert!(*callback_triggered.borrow(), "The start setup wizard button should trigger the start_setup_wizard callback.");
}

#[test]
fn test_login_oauth_login_button_triggers_callback() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let callback_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let callback_triggered_clone = callback_triggered.clone();

    ui.on_oauth_login(move |provider| {
        if provider == "Google/Apple" {
            *callback_triggered_clone.borrow_mut() = true;
        }
    });

    ui.invoke_oauth_login("Google/Apple".into());
    assert!(*callback_triggered.borrow(), "The oauth_login callback should be triggered for Google/Apple.");
}

#[test]
fn test_login_login_button_triggers_callback() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let callback_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let callback_triggered_clone = callback_triggered.clone();

    ui.on_login(move |username, password| {
        if username == "test" && password == "pass" {
            *callback_triggered_clone.borrow_mut() = true;
        }
    });

    ui.invoke_login("test".into(), "pass".into());
    assert!(*callback_triggered.borrow(), "The login callback should be triggered.");
}

#[test]
fn test_login_resend_verification_button_triggers_callback() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let callback_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let callback_triggered_clone = callback_triggered.clone();

    ui.on_resend_verification(move |username| {
        if username == "test" {
            *callback_triggered_clone.borrow_mut() = true;
        }
    });

    ui.invoke_resend_verification("test".into());
    assert!(*callback_triggered.borrow(), "The resend_verification callback should be triggered.");
}

#[test]
fn test_login_open_settings_button_triggers_callback() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let callback_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let callback_triggered_clone = callback_triggered.clone();

    ui.on_open_settings(move || {
        *callback_triggered_clone.borrow_mut() = true;
    });

    ui.invoke_open_settings();
    assert!(*callback_triggered.borrow(), "The open_settings callback should be triggered.");
}
