use crate::app;

#[test]
fn test_e2e_full_onboarding_journey() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.borrow_mut() = true;
    });

    let setup_wizard_launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let setup_wizard_launched_clone = setup_wizard_launched.clone();

    login_ui.on_start_setup_wizard(move || {
        *setup_wizard_launched_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "secure123".into());
    login_ui.invoke_start_setup_wizard();
    assert!(*setup_wizard_launched.borrow());

    let wizard_ui = app::SetupWizard::new().unwrap();

    assert_eq!(wizard_ui.get_step(), 0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.set_company_name("My E2E Bakery".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.invoke_select_business_type("Physical".into());

    let launch_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launch_called_clone = launch_called.clone();

    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _pp2, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Physical");
        assert_eq!(cn, "My E2E Bakery");
        *launch_called_clone.borrow_mut() = true;
    });

    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    wizard_ui.invoke_launch("Physical".into(), "My E2E Bakery".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launch_called.borrow(), "Wizard launch successfully executed");
}
