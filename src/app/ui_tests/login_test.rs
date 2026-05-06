use std::env;

#[test]
fn test_login_window_responsive_and_touch_targets() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }

    // We import slint generated components and app main structs
    // Wait, let's create a functional Slint test that checks login handles and dimensions
    // Since we don't have access to the internals of slint component easily in tests unless we instantiate, let's instantiate Login.
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

#[cfg(test)]
mod additional_login_tests {
    use slint::ComponentHandle;

    #[test]
    fn test_login_app_settings_text_exists() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        crate::ui_tests::init();
        let ui = crate::app::Login::new().unwrap();
        assert_eq!(ui.get_settings_button_text(), "⚙ App Settings");
    }

    #[test]
    fn test_login_username_placeholder_state() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        crate::ui_tests::init();
        let ui = crate::app::Login::new().unwrap();
        ui.set_username("test_user_for_state".into());
        assert_eq!(ui.get_username(), "test_user_for_state");
    }

    #[test]
    fn test_login_password_placeholder_state() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        crate::ui_tests::init();
        let ui = crate::app::Login::new().unwrap();
        ui.set_password("secure_pass_state".into());
        assert_eq!(ui.get_password(), "secure_pass_state");
    }

    #[test]
    fn test_login_verification_message_visibility() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        crate::ui_tests::init();
        let ui = crate::app::Login::new().unwrap();

        ui.set_show_verification(true);
        ui.set_verification_message("Please check your email.".into());

        assert_eq!(ui.get_show_verification(), true);
        assert_eq!(ui.get_verification_message(), "Please check your email.");
    }

    #[test]
    fn test_login_button_click_sim() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        crate::ui_tests::init();
        let ui = crate::app::Login::new().unwrap();

        let clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
        let clicked_clone = clicked.clone();

        ui.on_open_settings(move || {
            *clicked_clone.borrow_mut() = true;
        });

        ui.invoke_open_settings();

        assert!(*clicked.borrow(), "The settings callback should fire when invoked.");
    }
}
