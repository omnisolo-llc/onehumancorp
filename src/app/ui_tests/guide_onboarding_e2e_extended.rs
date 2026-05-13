#[test]
fn test_maya_baker_full_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    // Step 0 -> 1
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.invoke_select_business_type("Restaurant / Food".into());
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.set_company_name("Maya's Bakery".into());
    wizard_ui.set_company_description("Fresh breads and pastries".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    wizard_ui.invoke_toggle_sell_food();
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 4);

    wizard_ui.invoke_select_payment_pref("in_person".into());
    assert_eq!(wizard_ui.get_step(), 5);

    wizard_ui.set_admin_email("maya@bakery.example.com".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 6);

    wizard_ui.invoke_select_template("Rustic".into());
    assert_eq!(wizard_ui.get_step(), 7);

    wizard_ui.set_product_name("Sourdough Loaf".into());
    wizard_ui.set_product_price("8.50".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 8);

    wizard_ui.invoke_select_domain("mayasbakery.ohc.app".into());
    assert_eq!(wizard_ui.get_step(), 9);

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, pp, _ae, _wt, pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Restaurant / Food");
        assert_eq!(cn, "Maya's Bakery");
        assert_eq!(pp, "in_person");
        assert_eq!(pn, "Sourdough Loaf");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Restaurant / Food".into(), "Maya's Bakery".into(), "Fresh breads and pastries".into(), "in_person".into(), "maya@bakery.example.com".into(),
        "Rustic".into(), "Sourdough Loaf".into(), "8.50".into(), "mayasbakery.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow(), "Maya baker launch failed");
}

#[test]
fn test_carlos_handyman_full_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.invoke_select_business_type("Service Business".into());
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.set_company_name("Carlos Fixes It".into());
    wizard_ui.set_company_description("Local handyman services".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    wizard_ui.invoke_toggle_sell_services();
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 4);

    wizard_ui.invoke_select_payment_pref("in_person".into());
    assert_eq!(wizard_ui.get_step(), 5);

    wizard_ui.set_admin_email("carlos@fixes.example.com".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 6);

    wizard_ui.invoke_select_template("Bold".into());
    assert_eq!(wizard_ui.get_step(), 7);

    wizard_ui.set_product_name("1 Hour Repair".into());
    wizard_ui.set_product_price("60.00".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 8);

    wizard_ui.invoke_select_domain("carlosfixes.ohc.app".into());
    assert_eq!(wizard_ui.get_step(), 9);

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Service Business");
        assert_eq!(cn, "Carlos Fixes It");
        assert_eq!(pn, "1 Hour Repair");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Service Business".into(), "Carlos Fixes It".into(), "Local handyman services".into(), "in_person".into(), "carlos@fixes.example.com".into(),
        "Bold".into(), "1 Hour Repair".into(), "60.00".into(), "carlosfixes.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow(), "Carlos handyman launch failed");
}

#[test]
fn test_jessica_freelance_writer_full_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.invoke_select_business_type("Creative / Portfolio".into());
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.set_company_name("Jessica Writes".into());
    wizard_ui.set_company_description("Freelance copywriting".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    wizard_ui.invoke_toggle_sell_services();
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 4);

    wizard_ui.invoke_select_payment_pref("online".into());
    assert_eq!(wizard_ui.get_step(), 5);

    wizard_ui.set_admin_email("jessica@writes.example.com".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 6);

    wizard_ui.invoke_select_template("Elegant".into());
    assert_eq!(wizard_ui.get_step(), 7);

    wizard_ui.set_product_name("Blog Post".into());
    wizard_ui.set_product_price("200.00".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 8);

    wizard_ui.invoke_select_domain("jessicawrites.ohc.app".into());
    assert_eq!(wizard_ui.get_step(), 9);

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Creative / Portfolio");
        assert_eq!(cn, "Jessica Writes");
        assert_eq!(pn, "Blog Post");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Creative / Portfolio".into(), "Jessica Writes".into(), "Freelance copywriting".into(), "online".into(), "jessica@writes.example.com".into(),
        "Elegant".into(), "Blog Post".into(), "200.00".into(), "jessicawrites.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow(), "Jessica writer launch failed");
}

#[test]
fn test_david_fitness_coach_full_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.invoke_select_business_type("Local Business".into());
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.set_company_name("David Fitness".into());
    wizard_ui.set_company_description("Personal fitness coaching".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    wizard_ui.invoke_toggle_sell_subscriptions();
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 4);

    wizard_ui.invoke_select_payment_pref("both".into());
    assert_eq!(wizard_ui.get_step(), 5);

    wizard_ui.set_admin_email("david@fitness.example.com".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 6);

    wizard_ui.invoke_select_template("Dynamic".into());
    assert_eq!(wizard_ui.get_step(), 7);

    wizard_ui.set_product_name("Monthly Coaching".into());
    wizard_ui.set_product_price("150.00".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 8);

    wizard_ui.invoke_select_domain("davidfitness.ohc.app".into());
    assert_eq!(wizard_ui.get_step(), 9);

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Local Business");
        assert_eq!(cn, "David Fitness");
        assert_eq!(pn, "Monthly Coaching");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Local Business".into(), "David Fitness".into(), "Personal fitness coaching".into(), "both".into(), "david@fitness.example.com".into(),
        "Dynamic".into(), "Monthly Coaching".into(), "150.00".into(), "davidfitness.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow(), "David fitness launch failed");
}

#[test]
fn test_emma_clothing_boutique_full_flow() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();

    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    wizard_ui.invoke_select_business_type("Online Store".into());
    assert_eq!(wizard_ui.get_step(), 2);

    wizard_ui.set_company_name("Emma's Boutique".into());
    wizard_ui.set_company_description("Trendy women's clothing".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    wizard_ui.invoke_toggle_sell_physical();
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 4);

    wizard_ui.invoke_select_payment_pref("online".into());
    assert_eq!(wizard_ui.get_step(), 5);

    wizard_ui.set_admin_email("emma@boutique.example.com".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 6);

    wizard_ui.invoke_select_template("Chic".into());
    assert_eq!(wizard_ui.get_step(), 7);

    wizard_ui.set_product_name("Summer Dress".into());
    wizard_ui.set_product_price("45.99".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 8);

    wizard_ui.invoke_select_domain("emmasboutique.ohc.app".into());
    assert_eq!(wizard_ui.get_step(), 9);

    let launched = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launched_clone = launched.clone();
    wizard_ui.on_launch(move |bt, cn, _cd, _pp, _ae, _wt, pn, _price, _dc, _an, _ap, _pt| {
        assert_eq!(bt, "Online Store");
        assert_eq!(cn, "Emma's Boutique");
        assert_eq!(pn, "Summer Dress");
        *launched_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_launch(
        "Online Store".into(), "Emma's Boutique".into(), "Trendy women's clothing".into(), "online".into(), "emma@boutique.example.com".into(),
        "Chic".into(), "Summer Dress".into(), "45.99".into(), "emmasboutique.ohc.app".into(), "".into(), "".into(), "".into()
    );
    assert!(*launched.borrow(), "Emma boutique launch failed");
}
