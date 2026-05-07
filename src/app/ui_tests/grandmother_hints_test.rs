use crate::app;

#[test]
fn test_business_manager_hint() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::BusinessManager::new().unwrap();
    assert_eq!(ui.get_show_hint(), false);
    ui.set_show_hint(true);
    assert_eq!(ui.get_show_hint(), true);
}

#[test]
fn test_business_share_hint() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::BusinessShare::new().unwrap();
    assert_eq!(ui.get_show_hint(), false);
    ui.set_show_hint(true);
    assert_eq!(ui.get_show_hint(), true);
}

#[test]
fn test_grow_business_hint() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::GrowBusiness::new().unwrap();
    assert_eq!(ui.get_show_hint(), false);
    ui.set_show_hint(true);
    assert_eq!(ui.get_show_hint(), true);
}

#[test]
fn test_grow_business_advanced_toggle() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::GrowBusiness::new().unwrap();
    assert_eq!(ui.get_is_advanced(), false);
    ui.invoke_toggle_advanced();
}

#[test]
fn test_business_manager_step_toggle() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::BusinessManager::new().unwrap();
    assert_eq!(ui.get_step(), 0);
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);
}
