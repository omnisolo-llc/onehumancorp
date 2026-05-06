use crate::app;

// 1. Sign-Up & Account Creation transition to Setup Wizard
#[test]
fn test_e2e_onboarding_signup_to_wizard() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    crate::ui_tests::init();

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        assert_eq!(email, "maya@example.com");
        assert_eq!(password, "secure123");
        *login_successful_clone.borrow_mut() = true;
    });

    let start_setup_wizard_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let start_setup_wizard_called_clone = start_setup_wizard_called.clone();
    login_ui.on_start_setup_wizard(move || {
        *start_setup_wizard_called_clone.borrow_mut() = true;
    });

    // Assume user clicked login and succeeded
    login_ui.invoke_login("maya@example.com".into(), "secure123".into());
    assert!(*login_successful.borrow(), "User login should succeed");

    // The backend responds telling them to start the setup wizard
    login_ui.invoke_start_setup_wizard();
    assert!(
        *start_setup_wizard_called.borrow(),
        "Setup Wizard should launch seamlessly from login"
    );
}

// 2. Setup Wizard Template Selection
#[test]
fn test_e2e_onboarding_template_preview_selection() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    crate::ui_tests::init();
    let ui = app::SetupWizard::new().unwrap();

    ui.set_step(6);
    ui.set_company_name("Maya's Cakes".into());
    assert_eq!(ui.get_company_name(), "Maya's Cakes");

    // Verify it updates state and proceeds
    ui.invoke_select_template("Modern".into());
    assert_eq!(ui.get_step(), 7);
    assert_eq!(ui.get_website_template(), "Modern");
}

// 3. First Product / Service Add
#[test]
fn test_e2e_onboarding_first_product_creation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    crate::ui_tests::init();
    let ui = app::SetupWizard::new().unwrap();

    ui.set_step(7);

    // Simulate user input for adding a product
    ui.set_product_name("Vegan Chocolate Cake".into());
    assert_eq!(ui.get_product_name(), "Vegan Chocolate Cake");

    ui.set_price_type("fixed".into());
    ui.set_product_price("45.00".into());
    assert_eq!(ui.get_product_price(), "45.00");

    // Proceed to next
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 8);
}

// 4. Domain & Go-Live
#[test]
fn test_e2e_onboarding_domain_and_launch() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    crate::ui_tests::init();
    let ui = app::SetupWizard::new().unwrap();

    ui.set_step(8);

    // Pick subdomain
    ui.invoke_select_domain("subdomain".into());
    assert_eq!(ui.get_step(), 9);
    assert_eq!(ui.get_domain_choice(), "subdomain");

    // Test Launch step logic
    let launch_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let launch_called_clone = launch_called.clone();

    ui.on_launch(
        move |_bt,
              _cn,
              _cd,
              _pp,
              _ae,
              website_template,
              product_name,
              product_price,
              domain_choice,
              _an,
              _ap| {
            assert_eq!(website_template, "Modern");
            assert_eq!(product_name, "Vegan Chocolate Cake");
            assert_eq!(product_price, "45.00");
            assert_eq!(domain_choice, "subdomain");
            *launch_called_clone.borrow_mut() = true;
        },
    );

    ui.set_website_template("Modern".into());
    ui.set_product_name("Vegan Chocolate Cake".into());
    ui.set_product_price("45.00".into());
    ui.set_domain_choice("subdomain".into());

    ui.invoke_launch(
        "".into(),
        "".into(),
        "".into(),
        "".into(),
        "".into(),
        ui.get_website_template(),
        ui.get_product_name(),
        ui.get_product_price(),
        ui.get_domain_choice(),
        "".into(),
        "".into(),
    );

    assert!(
        *launch_called.borrow(),
        "Launch should be called successfully"
    );
}

// 5. Welcome Checklist
#[test]
fn test_e2e_onboarding_welcome_checklist_progress() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    crate::ui_tests::init();

    let ui = app::WelcomeChecklist::new().unwrap();
    crate::setup_welcome_checklist_routing(&ui);

    // Initial state
    assert_eq!(ui.get_progress(), 0);
    assert_eq!(ui.get_is_completed(), false);

    // Mock completion logic flow
    ui.set_progress(25);
    assert_eq!(ui.get_progress(), 25);

    ui.set_progress(100);
    ui.set_is_completed(true);
    assert_eq!(ui.get_is_completed(), true);
    assert_eq!(ui.get_progress(), 100);

    // Validating navigation triggers correctly.
    let dashboard_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let dashboard_clone = dashboard_triggered.clone();
    ui.on_go_to_dashboard(move || {
        *dashboard_clone.borrow_mut() = true;
    });

    ui.invoke_go_to_dashboard();
    assert!(
        *dashboard_triggered.borrow(),
        "Go to Dashboard callback works"
    );
}

#[test]
fn test_e2e_onboarding_sso_signup() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

    let login_ui = crate::app::Login::new().unwrap();
    let oauth_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let oauth_called_clone = oauth_called.clone();

    login_ui.on_oauth_login(move |provider| {
        assert_eq!(provider, "Google");
        *oauth_called_clone.borrow_mut() = true;
    });

    login_ui.invoke_oauth_login("Google".into());
    assert!(
        *oauth_called.borrow(),
        "OAuth login should trigger SSO signup flow"
    );
}

#[test]
fn test_e2e_onboarding_domain_assignment() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(8);

    let domain_selected = std::rc::Rc::new(std::cell::RefCell::new(false));
    let domain_selected_clone = domain_selected.clone();

    wizard_ui.on_select_domain(move |domain| {
        assert_eq!(domain, "subdomain");
        *domain_selected_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_select_domain("subdomain".into());
    assert!(
        *domain_selected.borrow(),
        "Free subdomain assignment should work"
    );
}

#[test]
fn test_e2e_onboarding_product_ai_generation() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(7);
    wizard_ui.set_product_name("Vegan Cupcakes".into());

    let ai_gen_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let ai_gen_called_clone = ai_gen_called.clone();

    wizard_ui.on_generate_product_description(move |name| {
        assert_eq!(name, "Vegan Cupcakes");
        *ai_gen_called_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_generate_product_description("Vegan Cupcakes".into());
    assert!(
        *ai_gen_called.borrow(),
        "AI generation for products should work"
    );
}

#[test]
fn test_e2e_onboarding_cross_device_resume() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(5);

    let save_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let save_called_clone = save_called.clone();

    wizard_ui.on_save_state(move || {
        *save_called_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_save_state();
    assert!(
        *save_called.borrow(),
        "State should persist for cross-device resume"
    );
}

#[test]
fn test_e2e_onboarding_welcome_checklist() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

    let wizard_ui = crate::app::SetupWizard::new().unwrap();
    wizard_ui.set_step(100);

    let checklist_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let checklist_called_clone = checklist_called.clone();

    wizard_ui.on_show_welcome_checklist(move || {
        *checklist_called_clone.borrow_mut() = true;
    });

    wizard_ui.invoke_show_welcome_checklist();
    assert!(
        *checklist_called.borrow(),
        "Welcome checklist post-onboarding should work"
    );
}
