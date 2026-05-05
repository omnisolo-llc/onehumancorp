use std::env;

#[test]
fn test_login_window_responsive_and_touch_targets() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();

    // Default width test
    let expected_default_width = 400.0;
    ui.window().set_size(slint::PhysicalSize::new(1440, 900));
    // When width is 1440, width - 64px is 1376. min(1376, 400) = 400
    assert_eq!(ui.get_login_card_width(), expected_default_width);

    // Mobile width test
    ui.window().set_size(slint::PhysicalSize::new(375, 812));
    // When width is 375, width - 64px is 311. min(311, 400) = 311
    assert_eq!(ui.get_login_card_width(), 311.0);
}

#[test]
fn test_login_sign_up_toggle_state() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();

    assert_eq!(ui.get_is_sign_up(), false);
    ui.set_is_sign_up(true);
    assert_eq!(ui.get_is_sign_up(), true);
}

#[test]
fn test_login_error_message_visibility() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();

    assert_eq!(ui.get_error_message(), "");
    ui.set_error_message("Invalid credentials".into());
    assert_eq!(ui.get_error_message(), "Invalid credentials");
}

#[test]
fn test_login_verification_message_visibility() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();

    assert_eq!(ui.get_show_verification(), false);
    assert_eq!(ui.get_verification_message(), "");

    ui.set_show_verification(true);
    ui.set_verification_message("Check email".into());

    assert_eq!(ui.get_show_verification(), true);
    assert_eq!(ui.get_verification_message(), "Check email");
}

#[test]
fn test_login_oauth_button_trigger() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let callback_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let callback_triggered_clone = callback_triggered.clone();

    ui.on_oauth_login(move |provider| {
        assert_eq!(provider, "SSO");
        *callback_triggered_clone.borrow_mut() = true;
    });

    ui.invoke_oauth_login("SSO".into());

    assert!(*callback_triggered.borrow(), "The oauth login button should trigger the callback.");
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
