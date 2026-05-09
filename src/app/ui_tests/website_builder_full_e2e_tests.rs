use crate::app;

#[test]
fn test_e2e_wizard_website_builder_full_journey() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        assert_eq!(email, "test@example.com");
        assert_eq!(password, "password123");
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "User login should be successful");

    let wizard_ui = app::WebsiteBuilder::new().unwrap();
    assert_eq!(wizard_ui.get_step(), 0);

    let publish_site_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let publish_site_called_clone = publish_site_called.clone();

    wizard_ui.on_publish_site(move |_t, _c, _pn, _pp, _pd, _dc| {
        *publish_site_called_clone.borrow_mut() = true;
    });

    // Step 0: Template Selection
    wizard_ui.set_selected_template("Modern".into());
    wizard_ui.set_step(1);
    assert_eq!(wizard_ui.get_step(), 1);

    // Step 1: Branding
    wizard_ui.set_primary_color("#34C759".into());
    wizard_ui.set_step(2);
    assert_eq!(wizard_ui.get_step(), 2);

    // Step 2: First Product
    wizard_ui.set_product_name("Vegan Chocolate Cake".into());
    wizard_ui.set_product_price("25.00".into());
    wizard_ui.set_step(3);
    assert_eq!(wizard_ui.get_step(), 3);

    // Step 3: Connect Domain
    wizard_ui.set_domain_choice("subdomain".into());
    wizard_ui.set_step(4);
    assert_eq!(wizard_ui.get_step(), 4);

    // Step 4: Publish
    wizard_ui.invoke_publish_site("Modern".into(), "#34C759".into(), "Vegan Chocolate Cake".into(), "25.00".into(), "".into(), "subdomain".into());
    assert!(*publish_site_called.borrow(), "Publish site should be clicked and trigger publish_site callback");
}
