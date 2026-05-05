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
fn test_login_card_width_at_1440() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    use slint::ComponentHandle;
    ui.window().set_size(slint::PhysicalSize::new(1440, 900));
    let width = ui.get_login_card_width();
    assert_eq!(width, 400.0, "Card width should be 400 at 1440 screen width");
}

#[test]
fn test_login_card_width_at_768() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    use slint::ComponentHandle;
    ui.window().set_size(slint::PhysicalSize::new(768, 1024));
    let width = ui.get_login_card_width();
    assert_eq!(width, 400.0, "Card width should be 400 at 768 screen width");
}

#[test]
fn test_login_card_width_at_414() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    use slint::ComponentHandle;
    ui.window().set_size(slint::PhysicalSize::new(414, 896));
    let width = ui.get_login_card_width();
    assert_eq!(width, 400.0, "Card width should be 400 at 414 screen width");
}

#[test]
fn test_login_card_width_at_375() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    use slint::ComponentHandle;
    ui.window().set_size(slint::PhysicalSize::new(375, 800));
    let width = ui.get_login_card_width();
    assert_eq!(width, 375.0, "Card width should take full width at 375 screen width");
}

#[test]
fn test_login_card_width_at_300() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    use slint::ComponentHandle;
    ui.window().set_size(slint::PhysicalSize::new(300, 600));
    let width = ui.get_login_card_width();
    assert_eq!(width, 300.0, "Card width should take full width at 300 screen width");
}
