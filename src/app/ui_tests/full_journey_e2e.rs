use crate::app;

#[test]
fn test_e2e_full_onboarding_journey() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. Start at Login
    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        assert_eq!(email, "test@example.com");
        assert_eq!(password, "secure123");
        *login_successful_clone.borrow_mut() = true;
    });

    let setup_wizard_launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let setup_wizard_launched_clone = setup_wizard_launched.clone();

    login_ui.on_start_setup_wizard(move || {
        *setup_wizard_launched_clone.borrow_mut() = true;
    });

    // Action: User logs in
    login_ui.invoke_login("test@example.com".into(), "secure123".into());
    assert!(*login_successful.borrow(), "Login logic should succeed");

    // Simulate backend response triggering setup wizard
    login_ui.invoke_start_setup_wizard();
    assert!(*setup_wizard_launched.borrow(), "Setup wizard transition triggered");

    // 2. Setup Wizard E2E Flow
    let wizard_ui = app::SetupWizard::new().unwrap();

    assert_eq!(wizard_ui.get_step(), 0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.invoke_select_business_type("Online Store".into());
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.set_company_name("My E2E Bakery".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    wizard_ui.invoke_toggle_sell_physical();
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 4);

    wizard_ui.invoke_select_payment_pref("online".into());
    assert_eq!(wizard_ui.get_step(), 5);

    wizard_ui.set_admin_email("test@example.com".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 6);

    wizard_ui.invoke_select_template("Modern".into());
    assert_eq!(wizard_ui.get_step(), 7);

    wizard_ui.set_product_name("Cupcake".into());
    wizard_ui.set_product_price("5.00".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 8);

    wizard_ui.invoke_select_domain("mye2ebakery.ohc.app".into());
    assert_eq!(wizard_ui.get_step(), 9);

    let launch_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launch_called_clone = launch_called.clone();

    wizard_ui.on_launch(move |bt, cn, _cd, pp, ae, wt, pn, price, dc, _an, _ap, _pt| {
        assert_eq!(bt, "Online Store");
        assert_eq!(cn, "My E2E Bakery");
        assert_eq!(pp, "online");
        assert_eq!(ae, "test@example.com");
        assert_eq!(wt, "Modern");
        assert_eq!(pn, "Cupcake");
        assert_eq!(price, "5.00");
        assert_eq!(dc, "mye2ebakery.ohc.app");
        *launch_called_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Online Store".into(), "My E2E Bakery".into(), "".into(), "online".into(), "test@example.com".into(),
        "Modern".into(), "Cupcake".into(), "5.00".into(), "mye2ebakery.ohc.app".into(), "".into(), "".into(), "".into()
    );

    assert!(*launch_called.borrow(), "Wizard launch successfully executed");

    // 3. Transition to Dashboard
    let dashboard_ui = app::Dashboard::new().unwrap();
    assert_eq!(dashboard_ui.get_active_helpers_count(), 0); // Verify default state
}
