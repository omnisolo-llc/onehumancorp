use crate::app;
use slint::Model;


#[test]
fn test_business_share_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let share_ui = app::BusinessShare::new().unwrap();

    // Assert visual truth / token truth: test_title exists and matches
    assert_eq!(share_ui.get_test_title(), slint::SharedString::from("Share my business"));

    let copy_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let copy_clone = copy_called.clone();
    share_ui.on_copy_link(move || {
        *copy_clone.borrow_mut() = true;
    });

    share_ui.invoke_copy_link();
    assert!(*copy_called.borrow(), "Copy link callback must be triggered");
}

#[test]
fn test_help_center_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let help_ui = app::HelpCenter::new().unwrap();
    assert!(help_ui.get_articles().row_count() > 0, "Articles should not be empty");
}

#[test]
fn test_business_manager_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let manager_ui = app::BusinessManager::new().unwrap();
    assert_eq!(manager_ui.get_current_view(), slint::SharedString::from("list"));
    manager_ui.invoke_action_add_new();
    assert_eq!(manager_ui.get_current_view(), slint::SharedString::from("add"));
}

#[test]
fn test_setup_wizard_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = app::SetupWizard::new().unwrap();
    assert_eq!(wizard_ui.get_step(), 0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);
}

#[test]
fn test_dashboard_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let dashboard_ui = app::Dashboard::new().unwrap();

    let executed = std::rc::Rc::new(std::cell::RefCell::new(false));
    let executed_clone = executed.clone();

    dashboard_ui.on_action_view_orders(move || {
        *executed_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_view_orders();
    assert!(*executed.borrow(), "Orders action must be triggered");
}
