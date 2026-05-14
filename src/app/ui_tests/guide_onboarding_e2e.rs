use crate::app;

#[test]
fn test_sam_photographer_onboarding_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    // Step 0 -> 1
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.set_company_name("Sam's Studio".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.invoke_select_business_type("Service".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Service");
        assert_eq!(cn, "Sam's Studio");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch("Service".into(), "Sam's Studio".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launched.borrow(), "Sam's onboarding launch failed");
}

#[test]
fn test_alex_fitness_coach_onboarding_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.set_company_name("Alex Fitness".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.invoke_select_business_type("Service".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Service");
        assert_eq!(cn, "Alex Fitness");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch("Service".into(), "Alex Fitness".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launched.borrow(), "Alex's onboarding launch failed");
}
