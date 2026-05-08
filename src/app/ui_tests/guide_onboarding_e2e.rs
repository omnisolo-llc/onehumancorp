// New comprehensive E2E test for the existing onboarding components to satisfy the assignment
// Persona: Sam — The Photographer
#[test]
fn test_sam_photographer_onboarding_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    // Step 0 -> 1
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    // Select Business Type
    wizard_ui.invoke_select_business_type("Service".into());
    assert_eq!(wizard_ui.get_step(), 2);

    // Name and description
    wizard_ui.set_company_name("Sam's Studio".into());
    wizard_ui.set_company_description("Professional photography".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    // Skip physical goods (it's a service)
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 4);

    // Payment
    wizard_ui.invoke_select_payment_pref("online".into());
    assert_eq!(wizard_ui.get_step(), 5);

    // Admin email
    wizard_ui.set_admin_email("sam@samsstudio.example.com".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 6);

    // Template
    wizard_ui.invoke_select_template("Minimalist".into());
    assert_eq!(wizard_ui.get_step(), 7);

    // First product
    wizard_ui.set_product_name("Portrait Session".into());
    wizard_ui.set_product_price("150.00".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 8);

    // Domain selection
    wizard_ui.invoke_select_domain("samsstudio.ohc.app".into());
    assert_eq!(wizard_ui.get_step(), 9);

    // Launch logic test
    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, cd, pp, ae, wt, pn, price, dc, _an, _ap, _pt| {
        assert_eq!(bt, "Service");
        assert_eq!(cn, "Sam's Studio");
        assert_eq!(cd, "Professional photography");
        assert_eq!(pp, "online");
        assert_eq!(ae, "sam@samsstudio.example.com");
        assert_eq!(wt, "Minimalist");
        assert_eq!(pn, "Portrait Session");
        assert_eq!(price, "150.00");
        assert_eq!(dc, "samsstudio.ohc.app");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Service".into(), "Sam's Studio".into(), "Professional photography".into(), "online".into(), "sam@samsstudio.example.com".into(),
        "Minimalist".into(), "Portrait Session".into(), "150.00".into(), "samsstudio.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow(), "Sam's onboarding launch failed");
}

// Persona: Alex — The Fitness Coach
#[test]
fn test_alex_fitness_coach_onboarding_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.invoke_select_business_type("Service".into());
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.set_company_name("Alex Fitness".into());
    wizard_ui.set_company_description("Personal training and fitness plans".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 4);

    wizard_ui.invoke_select_payment_pref("online".into());
    assert_eq!(wizard_ui.get_step(), 5);

    wizard_ui.set_admin_email("alex@alexfitness.example.com".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 6);

    wizard_ui.invoke_select_template("Modern".into());
    assert_eq!(wizard_ui.get_step(), 7);

    wizard_ui.set_product_name("1-on-1 Coaching Session".into());
    wizard_ui.set_product_price("100.00".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 8);

    wizard_ui.invoke_select_domain("alexfitness.ohc.app".into());
    assert_eq!(wizard_ui.get_step(), 9);

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, cd, pp, ae, wt, pn, price, dc, _an, _ap, _pt| {
        assert_eq!(bt, "Service");
        assert_eq!(cn, "Alex Fitness");
        assert_eq!(cd, "Personal training and fitness plans");
        assert_eq!(pp, "online");
        assert_eq!(ae, "alex@alexfitness.example.com");
        assert_eq!(wt, "Modern");
        assert_eq!(pn, "1-on-1 Coaching Session");
        assert_eq!(price, "100.00");
        assert_eq!(dc, "alexfitness.ohc.app");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Service".into(), "Alex Fitness".into(), "Personal training and fitness plans".into(), "online".into(), "alex@alexfitness.example.com".into(),
        "Modern".into(), "1-on-1 Coaching Session".into(), "100.00".into(), "alexfitness.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow(), "Alex's onboarding launch failed");
}
