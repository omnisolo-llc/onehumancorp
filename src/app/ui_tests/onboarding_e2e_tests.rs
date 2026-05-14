use crate::app;

#[test]
fn test_maya_baker_onboarding_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.set_company_name("Maya's Cakes".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.invoke_select_business_type("Physical".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Physical");
        assert_eq!(cn, "Maya's Cakes");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch("Physical".into(), "Maya's Cakes".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launched.borrow(), "Maya's onboarding launch failed");
}

#[test]
fn test_e2e_onboarding_domain_and_launch() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Test Domain".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Digital".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Digital");
        assert_eq!(cn, "Test Domain");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch("Digital".into(), "Test Domain".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launched.borrow(), "Domain onboarding launch failed");
}

#[test]
fn test_e2e_onboarding_template_preview_selection() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Test Template".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Digital".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Digital");
        assert_eq!(cn, "Test Template");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch("Digital".into(), "Test Template".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launched.borrow(), "Template onboarding launch failed");
}
