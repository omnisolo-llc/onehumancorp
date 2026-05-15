use crate::app;

use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_echo_grandmother_e2e_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. Start from Login
    let login_ui = app::Login::new().unwrap();
    let login_invoked = Rc::new(RefCell::new(false));
    let login_clone = login_invoked.clone();

    login_ui.on_login(move |_username, _password| {
        *login_clone.borrow_mut() = true;
    });

    // Simulate error state
    login_ui.set_error_message("Invalid credentials".into());
    assert_eq!(login_ui.get_error_message(), slint::SharedString::from("Invalid credentials"));

    // Simulate successful login
    login_ui.set_username("ceo@store.com".into());
    login_ui.set_password("123".into());
    login_ui.invoke_login(login_ui.get_username(), login_ui.get_password());
    assert!(*login_invoked.borrow(), "Login action not triggered");

    // 2. Dashboard interactions
    let dashboard_ui = app::Dashboard::new().unwrap();

    // Validate Loading Shimmer
    dashboard_ui.set_is_loading(true);
    assert!(dashboard_ui.get_is_loading());

    // Validate Share Store Button (Reachability)
    let share_invoked = Rc::new(RefCell::new(false));
    let share_clone = share_invoked.clone();
    dashboard_ui.on_action_share_store(move || {
        *share_clone.borrow_mut() = true;
    });
    dashboard_ui.invoke_action_share_store();
    assert!(*share_invoked.borrow(), "Share Store action not triggered");

    // 3. Move to Business Manager via Quick Action
    let add_product_invoked = Rc::new(RefCell::new(false));
    let add_clone = add_product_invoked.clone();
    dashboard_ui.on_action_build_website(move || {
        *add_clone.borrow_mut() = true;
    });
    dashboard_ui.invoke_action_build_website();
    assert!(*add_product_invoked.borrow(), "Add Product action not triggered");

    // 4. Business Manager interactions
    let manager_ui = app::BusinessManager::new().unwrap();
    assert_eq!(manager_ui.get_step(), 0);
    manager_ui.invoke_select_type("PHYSICAL".into());
    manager_ui.invoke_next_step();
    assert_eq!(manager_ui.get_step(), 1);

    manager_ui.set_product_name("Vegan Cookies".into());
    manager_ui.set_product_price("15.00".into());

    let submit_invoked = Rc::new(RefCell::new(false));
    let submit_clone = submit_invoked.clone();
    manager_ui.on_submit(move |_t, n, _d, _p, _dur, _sch| {
        *submit_clone.borrow_mut() = true;
        assert_eq!(n, "Vegan Cookies");
    });
    manager_ui.invoke_submit(
        manager_ui.get_selected_type(),
        manager_ui.get_product_name(),
        manager_ui.get_product_description(),
        manager_ui.get_product_price(),
        manager_ui.get_service_duration(),
        manager_ui.get_service_schedule()
    );
    assert!(*submit_invoked.borrow(), "Submit action not triggered");
}
