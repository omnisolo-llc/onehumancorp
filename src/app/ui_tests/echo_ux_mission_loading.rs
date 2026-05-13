use crate::app;
use slint::ComponentHandle;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_business_manager_loading_initial_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let ui = app::BusinessManager::new().unwrap();
    // Default should not be loading
    assert!(!ui.get_is_loading(), "Business Manager should not be loading by default");
}

#[test]
fn test_business_manager_loading_toggle() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let ui = app::BusinessManager::new().unwrap();

    ui.set_is_loading(true);
    assert!(ui.get_is_loading(), "Business Manager should reflect loading state when toggled");

    ui.set_is_loading(false);
    assert!(!ui.get_is_loading(), "Business Manager should reflect non-loading state when toggled off");
}

#[test]
fn test_business_manager_loading_preserves_view() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let ui = app::BusinessManager::new().unwrap();

    ui.set_current_view("list".into());
    ui.set_is_loading(true);
    assert_eq!(ui.get_current_view(), slint::SharedString::from("list"), "Current view should be preserved during loading state");
}

#[test]
fn test_business_manager_loading_preserves_step() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let ui = app::BusinessManager::new().unwrap();

    ui.set_step(1);
    ui.set_is_loading(true);
    assert_eq!(ui.get_step(), 1, "Step should be preserved during loading state");
}

#[test]
fn test_business_manager_loading_e2e_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Simulate navigation from Dashboard
    let dashboard_ui = app::Dashboard::new().unwrap();

    let add_product_invoked = Rc::new(RefCell::new(false));
    let add_clone = add_product_invoked.clone();
    dashboard_ui.on_action_add_product(move || {
        *add_clone.borrow_mut() = true;
    });
    dashboard_ui.invoke_action_add_product();
    assert!(*add_product_invoked.borrow(), "Add Product action not triggered");

    // Once in Business Manager, check loading
    let ui = app::BusinessManager::new().unwrap();
    ui.set_is_loading(true);
    assert!(ui.get_is_loading(), "Business Manager loading state could not be set");

    ui.set_is_loading(false);
    assert!(!ui.get_is_loading(), "Business Manager loading state could not be unset");
}
