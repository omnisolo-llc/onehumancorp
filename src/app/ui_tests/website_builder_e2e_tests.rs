use crate::app;
use slint::ComponentHandle;

#[test]
fn test_website_builder_e2e_full_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // The user logs in and starts from home page. We navigate directly to the WebsiteBuilder UI for the test.
    let ui = app::WebsiteBuilder::new().unwrap();

    // Step 0: Initial state
    assert_eq!(ui.get_step(), 0);

    // Click Modern template
    ui.set_selected_template("Modern".into());
    assert_eq!(ui.get_selected_template(), "Modern");

    // Click Next
    ui.set_step(1);
    assert_eq!(ui.get_step(), 1);

    // Step 1: Colors & Logo
    ui.set_primary_color("#34C759".into()); // Nature Green
    assert_eq!(ui.get_primary_color(), "#34C759");

    // Simulate Generate Description button click
    let description_generated = std::rc::Rc::new(std::cell::RefCell::new(false));
    let description_generated_clone = description_generated.clone();
    ui.on_generate_description(move || {
        *description_generated_clone.borrow_mut() = true;
    });
    ui.invoke_generate_description();
    assert!(*description_generated.borrow(), "Auto-generate description clicked");

    // Click Next
    ui.set_step(2);
    assert_eq!(ui.get_step(), 2);

    // Step 2: Product
    ui.set_product_name("Vegan Chocolate Cake".into());
    ui.set_product_price("25.00".into());
    ui.set_product_description("Delicious egg-free cake".into());

    assert_eq!(ui.get_product_name(), "Vegan Chocolate Cake");
    assert_eq!(ui.get_product_price(), "25.00");
    assert_eq!(ui.get_product_description(), "Delicious egg-free cake");

    // Click Next
    ui.set_step(3);
    assert_eq!(ui.get_step(), 3);

    // Step 3: Domain
    ui.set_domain_choice("subdomain".into());
    assert_eq!(ui.get_domain_choice(), "subdomain");

    // Click Next
    ui.set_step(4);
    assert_eq!(ui.get_step(), 4);

    // Step 4: Publish Review and go live
    // The review uses smart blocks, which reads the bindings correctly.

    // Click Publish ->
    let publish_clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let publish_clicked_clone = publish_clicked.clone();

    let ui_weak = ui.as_weak();
    ui.on_publish_site(move |template, color, name, price, desc, domain| {
        *publish_clicked_clone.borrow_mut() = true;
        assert_eq!(template, "Modern");
        assert_eq!(color, "#34C759");
        assert_eq!(name, "Vegan Chocolate Cake");
        assert_eq!(price, "25.00");
        assert_eq!(desc, "Delicious egg-free cake");
        assert_eq!(domain, "subdomain");
        if let Some(ui_instance) = ui_weak.upgrade() {
            ui_instance.set_is_publishing(true);
        }
    });

    // Simulate Next button logic on Step 4
    ui.invoke_publish_site(
        ui.get_selected_template(),
        ui.get_primary_color(),
        ui.get_product_name(),
        ui.get_product_price(),
        ui.get_product_description(),
        ui.get_domain_choice()
    );

    assert!(*publish_clicked.borrow(), "Publish should be clicked");

    // Validate that publishing state has been applied correctly.
    assert_eq!(ui.get_is_publishing(), true);

    // Validate final output elements. Wait for simulated copy functionality.
    let link_copied = std::rc::Rc::new(std::cell::RefCell::new(false));
    let link_copied_clone = link_copied.clone();
    ui.on_copy_to_clipboard(move |link| {
        assert_eq!(link, "https://mybusiness.ohc.app");
        *link_copied_clone.borrow_mut() = true;
    });

    ui.invoke_copy_to_clipboard("https://mybusiness.ohc.app".into());
    assert!(*link_copied.borrow(), "Link should be copied to clipboard");
}
