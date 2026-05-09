#[test]
fn test_alice_freelance_designer_onboarding_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    // Step 0 -> 1
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    // Select Business Type
    wizard_ui.invoke_select_business_type("Digital".into());
    assert_eq!(wizard_ui.get_step(), 2);

    // Name and description
    wizard_ui.set_company_name("Alice Design".into());
    wizard_ui.set_company_description("Freelance UI/UX design".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    // Skip physical goods
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 4);

    // Payment
    wizard_ui.invoke_select_payment_pref("online".into());
    assert_eq!(wizard_ui.get_step(), 5);

    // Admin email
    wizard_ui.set_admin_email("alice@alicedesign.example.com".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 6);

    // Template
    wizard_ui.invoke_select_template("Creative".into());
    assert_eq!(wizard_ui.get_step(), 7);

    // First product
    wizard_ui.set_product_name("UI Kit".into());
    wizard_ui.set_product_price("99.00".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 8);

    // Domain selection
    wizard_ui.invoke_select_domain("alicedesign.ohc.app".into());
    assert_eq!(wizard_ui.get_step(), 9);

    // Launch logic test
    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, cd, pp, ae, wt, pn, price, dc, _an, _ap, _pt| {
        assert_eq!(bt, "Digital");
        assert_eq!(cn, "Alice Design");
        assert_eq!(cd, "Freelance UI/UX design");
        assert_eq!(pp, "online");
        assert_eq!(ae, "alice@alicedesign.example.com");
        assert_eq!(wt, "Creative");
        assert_eq!(pn, "UI Kit");
        assert_eq!(price, "99.00");
        assert_eq!(dc, "alicedesign.ohc.app");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Digital".into(), "Alice Design".into(), "Freelance UI/UX design".into(), "online".into(), "alice@alicedesign.example.com".into(),
        "Creative".into(), "UI Kit".into(), "99.00".into(), "alicedesign.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow(), "Alice's onboarding launch failed");
}

#[test]
fn test_bob_coffee_shop_onboarding_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    // Step 0 -> 1
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();

    // Select Business Type
    wizard_ui.invoke_select_business_type("Food".into());

    // Name and description
    wizard_ui.set_company_name("Bob's Beans".into());
    wizard_ui.set_company_description("Local coffee shop".into());
    wizard_ui.invoke_next_step();

    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_payment_pref("in-person".into());

    wizard_ui.set_admin_email("bob@bobsbeans.example.com".into());
    wizard_ui.invoke_next_step();

    wizard_ui.invoke_select_template("Warm".into());

    wizard_ui.set_product_name("Espresso".into());
    wizard_ui.set_product_price("3.50".into());
    wizard_ui.invoke_next_step();

    wizard_ui.invoke_select_domain("bobsbeans.ohc.app".into());

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, cd, pp, ae, wt, pn, price, dc, _an, _ap, _pt| {
        assert_eq!(bt, "Food");
        assert_eq!(cn, "Bob's Beans");
        assert_eq!(cd, "Local coffee shop");
        assert_eq!(pp, "in-person");
        assert_eq!(ae, "bob@bobsbeans.example.com");
        assert_eq!(wt, "Warm");
        assert_eq!(pn, "Espresso");
        assert_eq!(price, "3.50");
        assert_eq!(dc, "bobsbeans.ohc.app");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Food".into(), "Bob's Beans".into(), "Local coffee shop".into(), "in-person".into(), "bob@bobsbeans.example.com".into(),
        "Warm".into(), "Espresso".into(), "3.50".into(), "bobsbeans.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow(), "Bob's onboarding launch failed");
}

#[test]
fn test_carol_consulting_onboarding_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Consulting".into());
    wizard_ui.set_company_name("Carol Consults".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_payment_pref("invoice".into());
    wizard_ui.set_admin_email("carol@example.com".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_template("Corporate".into());
    wizard_ui.set_product_name("1 Hour Session".into());
    wizard_ui.set_product_price("200.00".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_domain("carolconsults.ohc.app".into());

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, cd, pp, ae, wt, pn, price, dc, _an, _ap, _pt| {
        assert_eq!(bt, "Consulting");
        assert_eq!(cn, "Carol Consults");
        assert_eq!(cd, "");
        assert_eq!(pp, "invoice");
        assert_eq!(ae, "carol@example.com");
        assert_eq!(wt, "Corporate");
        assert_eq!(pn, "1 Hour Session");
        assert_eq!(price, "200.00");
        assert_eq!(dc, "carolconsults.ohc.app");
        *launched_clone.borrow_mut() = true;
    });
    wizard_ui.invoke_launch(
        "Consulting".into(), "Carol Consults".into(), "".into(), "invoice".into(), "carol@example.com".into(),
        "Corporate".into(), "1 Hour Session".into(), "200.00".into(), "carolconsults.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow());
}

#[test]
fn test_dave_dog_walking_onboarding_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Service".into());
    wizard_ui.set_company_name("Dave's Dogs".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_payment_pref("cash".into());
    wizard_ui.set_admin_email("dave@example.com".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_template("Playful".into());
    wizard_ui.set_product_name("Dog Walk".into());
    wizard_ui.set_product_price("25.00".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_domain("davesdogs.ohc.app".into());

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, cd, pp, ae, wt, pn, price, dc, _an, _ap, _pt| {
        assert_eq!(bt, "Service");
        assert_eq!(cn, "Dave's Dogs");
        assert_eq!(cd, "");
        assert_eq!(pp, "cash");
        assert_eq!(ae, "dave@example.com");
        assert_eq!(wt, "Playful");
        assert_eq!(pn, "Dog Walk");
        assert_eq!(price, "25.00");
        assert_eq!(dc, "davesdogs.ohc.app");
        *launched_clone.borrow_mut() = true;
    });
    wizard_ui.invoke_launch(
        "Service".into(), "Dave's Dogs".into(), "".into(), "cash".into(), "dave@example.com".into(),
        "Playful".into(), "Dog Walk".into(), "25.00".into(), "davesdogs.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow());
}

#[test]
fn test_eve_ecommerce_onboarding_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Retail".into());
    wizard_ui.set_company_name("Eve's Emporium".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_payment_pref("online".into());
    wizard_ui.set_admin_email("eve@example.com".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_template("Modern".into());
    wizard_ui.set_product_name("Handmade Soap".into());
    wizard_ui.set_product_price("12.00".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_domain("evesemporium.ohc.app".into());

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, cd, pp, ae, wt, pn, price, dc, _an, _ap, _pt| {
        assert_eq!(bt, "Retail");
        assert_eq!(cn, "Eve's Emporium");
        assert_eq!(cd, "");
        assert_eq!(pp, "online");
        assert_eq!(ae, "eve@example.com");
        assert_eq!(wt, "Modern");
        assert_eq!(pn, "Handmade Soap");
        assert_eq!(price, "12.00");
        assert_eq!(dc, "evesemporium.ohc.app");
        *launched_clone.borrow_mut() = true;
    });
    wizard_ui.invoke_launch(
        "Retail".into(), "Eve's Emporium".into(), "".into(), "online".into(), "eve@example.com".into(),
        "Modern".into(), "Handmade Soap".into(), "12.00".into(), "evesemporium.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow());
}
