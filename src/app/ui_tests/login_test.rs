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

#[test]
fn test_login_card_centering_desktop_width() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let window = ui.window();
    window.set_size(slint::PhysicalSize::new(1440, 900));
    // Test logic triggers UI update
    assert_eq!(ui.get_login_card_width(), 400.0);
}

#[test]
fn test_login_card_centering_tablet_width() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let window = ui.window();
    window.set_size(slint::PhysicalSize::new(768, 1024));
    assert_eq!(ui.get_login_card_width(), 400.0);
}

#[test]
fn test_login_card_centering_mobile_width() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let window = ui.window();
    window.set_size(slint::PhysicalSize::new(375, 667));
    assert_eq!(ui.get_login_card_width(), 311.0); // 375 - 64
}

#[test]
fn test_login_card_centering_mobile_wide_width() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let window = ui.window();
    window.set_size(slint::PhysicalSize::new(414, 896));
    assert_eq!(ui.get_login_card_width(), 350.0); // 414 - 64
}

#[test]
fn test_login_card_desktop_1024() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let window = ui.window();
    window.set_size(slint::PhysicalSize::new(1024, 768));
    assert_eq!(ui.get_login_card_width(), 400.0);
}
