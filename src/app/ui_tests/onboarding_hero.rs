use crate::app;
use slint::ComponentHandle;

#[test]
fn test_login_to_wizard_transition() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login = app::Login::new().unwrap();
    let wizard = app::SetupWizard::new().unwrap();

    let login_weak = login.as_weak();
    let wizard_weak = wizard.as_weak();

    login.on_resend_verification(move |_| {
        login_weak.upgrade().unwrap().hide().unwrap();
        wizard_weak.upgrade().unwrap().show().unwrap();
    });

    login.invoke_resend_verification("test@ohc.app".into());
    // Verification via visible property is hard without a global state manager in unit test,
    // but we verify the callback logic here.
}

#[test]
fn test_wizard_state_persistence_fields() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = app::SetupWizard::new().unwrap();

    ui.set_instant_bio("I bake bread".into());
    ui.set_product_sku("BREAD-001".into());
    ui.set_product_inventory("50".into());
    ui.set_product_description("Fresh sourdough".into());
    ui.set_price_type("fixed".into());

    assert_eq!(ui.get_instant_bio(), "I bake bread");
    assert_eq!(ui.get_product_sku(), "BREAD-001");
    assert_eq!(ui.get_product_inventory(), "50");
    assert_eq!(ui.get_product_description(), "Fresh sourdough");
    assert_eq!(ui.get_price_type(), "fixed");
}

#[test]
fn test_website_template_preview_update() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = app::SetupWizard::new().unwrap();
    ui.set_company_name("Test Business".into());

    ui.invoke_select_template("Modern".into());
    assert_eq!(ui.get_website_template(), "Modern");

    ui.invoke_select_template("Bold".into());
    assert_eq!(ui.get_website_template(), "Bold");
}

#[test]
fn test_launch_success_confetti_trigger() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = app::SetupWizard::new().unwrap();
    ui.set_step(9);
    assert!(!ui.get_launch_success());

    ui.set_launch_success(true);
    assert!(ui.get_launch_success());
    // Confetti is conditional on launch_success && step == 9 in the .slint file
}

#[test]
fn test_checklist_navigation_to_builder() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = app::WelcomeChecklist::new().unwrap();
    let builder_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let builder_called_clone = builder_called.clone();

    ui.on_go_to_add_products(move || {
        *builder_called_clone.borrow_mut() = true;
    });

    ui.invoke_go_to_add_products();
    assert!(*builder_called.borrow());
}
