use crate::app;
use slint::ComponentHandle;

#[test]
fn test_business_share_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let share_ui = app::BusinessShare::new().unwrap();

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
fn test_welcome_checklist_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let checklist_ui = app::WelcomeChecklist::new().unwrap();
    assert_eq!(checklist_ui.get_test_title(), slint::SharedString::from("Welcome Checklist"));

    let db_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let db_clone = db_called.clone();
    checklist_ui.on_go_to_dashboard(move || {
        *db_clone.borrow_mut() = true;
    });

    checklist_ui.invoke_go_to_dashboard();
    assert!(*db_called.borrow(), "Go to dashboard callback must be triggered");
}

#[test]
fn test_business_manager_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = app::BusinessManager::new().unwrap();

    let close_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let close_clone = close_called.clone();
    ui.on_close(move || {
        *close_clone.borrow_mut() = true;
    });

    ui.invoke_close();
    assert!(*close_called.borrow(), "Close callback must be triggered");
}

#[test]
fn test_setup_wizard_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = app::SetupWizard::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Setup Wizard"));

    ui.set_step(0);
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1, "Next step should progress the wizard");
}


#[test]
fn test_business_manager_full_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = app::BusinessManager::new().unwrap();

    // Start adding
    ui.invoke_action_add_new();
    assert_eq!(ui.get_current_view(), "add");
    assert_eq!(ui.get_step(), 0);

    // Select type
    ui.invoke_select_type("PHYSICAL".into());
    assert_eq!(ui.get_selected_type(), "PHYSICAL");

    // Next step
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);

    // Set details
    ui.set_product_name("New Shirt".into());
    ui.set_product_price("25.00".into());

    assert_eq!(ui.get_product_name(), "New Shirt");
}
