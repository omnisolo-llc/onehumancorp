use crate::app;

#[test]
fn test_e2e_wizard_pricing_full_journey() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        assert_eq!(email, "test@example.com");
        assert_eq!(password, "password123");
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "User login should be successful");

    let wizard_ui = app::Pricing::new().unwrap();
    assert_eq!(wizard_ui.get_step(), 0);

    let select_plan_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let select_plan_called_clone = select_plan_called.clone();

    wizard_ui.on_select_plan(move |plan| {
        assert_eq!(plan, "Pro");
        *select_plan_called_clone.borrow_mut() = true;
    });

    // Step 0: Dashboard usage viewing
    wizard_ui.set_step(1);
    assert_eq!(wizard_ui.get_step(), 1);

    // Step 1: Select Plan
    wizard_ui.invoke_select_plan("Pro".into());
    assert!(*select_plan_called.borrow(), "Select plan should be clicked and trigger select_plan callback");
}
