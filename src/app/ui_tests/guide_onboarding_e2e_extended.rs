use crate::app;

#[test]
fn test_carlos_handyman_full_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Carlos Handyman".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Service".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Service");
        assert_eq!(cn, "Carlos Handyman");
        *launched_clone.borrow_mut() = true;
    });
    wizard_ui.invoke_launch("Service".into(), "Carlos Handyman".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launched.borrow());
}

#[test]
fn test_david_fitness_coach_full_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("David Fitness".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Service".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Service");
        assert_eq!(cn, "David Fitness");
        *launched_clone.borrow_mut() = true;
    });
    wizard_ui.invoke_launch("Service".into(), "David Fitness".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launched.borrow());
}

#[test]
fn test_emma_clothing_boutique_full_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Emma Boutique".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Physical".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Physical");
        assert_eq!(cn, "Emma Boutique");
        *launched_clone.borrow_mut() = true;
    });
    wizard_ui.invoke_launch("Physical".into(), "Emma Boutique".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launched.borrow());
}

#[test]
fn test_jessica_freelance_writer_full_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Jessica Writer".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Service".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Service");
        assert_eq!(cn, "Jessica Writer");
        *launched_clone.borrow_mut() = true;
    });
    wizard_ui.invoke_launch("Service".into(), "Jessica Writer".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launched.borrow());
}

#[test]
fn test_maya_baker_full_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Maya Baker".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Physical".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, _pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Physical");
        assert_eq!(cn, "Maya Baker");
        *launched_clone.borrow_mut() = true;
    });
    wizard_ui.invoke_launch("Physical".into(), "Maya Baker".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*launched.borrow());
}
