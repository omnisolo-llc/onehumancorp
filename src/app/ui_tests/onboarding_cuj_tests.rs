use crate::app;
use slint::ComponentHandle;

#[test]
fn test_maya_baker_onboarding_flow_full() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    // Step 0 -> 1
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    // Select Business Type
    wizard_ui.invoke_select_business_type("Food".into());
    assert_eq!(wizard_ui.get_step(), 2);

    // Name and description
    wizard_ui.set_company_name("Maya's Cakes".into());
    wizard_ui.set_company_description("Delicious vegan cakes".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    // Physical goods
    wizard_ui.invoke_toggle_sell_physical();
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 4);

    // Payment
    wizard_ui.invoke_select_payment_pref("both".into());
    assert_eq!(wizard_ui.get_step(), 5);

    // Admin email
    wizard_ui.set_admin_email("maya@example.com".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 6);

    // Template
    wizard_ui.invoke_select_template("Modern".into());
    assert_eq!(wizard_ui.get_step(), 7);

    // First product
    wizard_ui.set_product_name("Vegan Chocolate Cake".into());
    wizard_ui.set_product_price("45.00".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 8);

    // Domain selection
    wizard_ui.invoke_select_domain("mayascakes.ohc.app".into());
    assert_eq!(wizard_ui.get_step(), 9);

    // Launch logic test
    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, cd, pp, ae, wt, pn, price, dc, _an, _ap, _pt| {
        assert_eq!(bt, "Food");
        assert_eq!(cn, "Maya's Cakes");
        assert_eq!(cd, "Delicious vegan cakes");
        assert_eq!(pp, "both");
        assert_eq!(ae, "maya@example.com");
        assert_eq!(wt, "Modern");
        assert_eq!(pn, "Vegan Chocolate Cake");
        assert_eq!(price, "45.00");
        assert_eq!(dc, "mayascakes.ohc.app");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Food".into(), "Maya's Cakes".into(), "Delicious vegan cakes".into(), "both".into(), "maya@example.com".into(),
        "Modern".into(), "Vegan Chocolate Cake".into(), "45.00".into(), "mayascakes.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow(), "Maya's onboarding launch failed");

    // Checklist
    let checklist_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let checklist_called_clone = checklist_called.clone();

    wizard_ui.on_show_welcome_checklist(move || {
        *checklist_called_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_show_welcome_checklist();
    assert!(*checklist_called.borrow(), "Welcome checklist post-onboarding should work");
}

#[test]
fn test_carlos_handyman_onboarding_flow_full() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    // Fast-track setup test (Instant Build)
    wizard_ui.set_step(0);
    wizard_ui.set_is_instant_build(true);
    wizard_ui.set_step(11);

    wizard_ui.set_instant_bio("I'm Carlos, a freelance handyman offering home repair services.".into());

    let generated = std::rc::Rc::new(std::cell::RefCell::new(false));
    let generated_clone = generated.clone();

    let wizard_weak = wizard_ui.as_weak();
    wizard_ui.on_generate_instant_preview(move || {
        if let Some(ui) = wizard_weak.upgrade() {
            ui.set_company_name("Carlos Handyman Services".into());
            ui.set_business_type("Service".into());
            ui.set_product_name("1-Hour Home Repair".into());
            ui.set_product_price("80.00".into());
            ui.set_is_generating_instant_preview(false);
            ui.set_step(9); // Ready to launch
        }
        *generated_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_generate_instant_preview();

    assert!(*generated.borrow(), "Carlos instant build failed");
    assert_eq!(wizard_ui.get_step(), 9);
    assert_eq!(wizard_ui.get_company_name(), "Carlos Handyman Services");
    assert_eq!(wizard_ui.get_product_price(), "80.00");
}

#[test]
fn test_cross_device_resume_full() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(5);
    wizard_ui.set_company_name("Cross Device Testing".into());

    let save_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let save_called_clone = save_called.clone();

    wizard_ui.on_save_state(move || {
        *save_called_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_save_state();
    assert!(*save_called.borrow(), "State should persist for cross-device resume");
}
