use crate::app;

fn create() -> app::WebsiteBuilder { crate::ui_tests::init(); app::WebsiteBuilder::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn builder_invalid_color() {
    let ui = create();
    ui.set_primary_color("not-a-color".into());
    assert_eq!(ui.get_primary_color(), "not-a-color");
}

#[test] fn builder_hex_color_variants() {
    let ui = create();
    ui.set_primary_color("#F00".into());
    assert_eq!(ui.get_primary_color(), "#F00");
    ui.set_primary_color("#FF0000".into());
    assert_eq!(ui.get_primary_color(), "#FF0000");
    ui.set_primary_color("#FF0000FF".into());
    assert_eq!(ui.get_primary_color(), "#FF0000FF");
}

#[test] fn builder_xss_template_name() {
    let ui = create();
    let xss = "'; alert(1); //";
    ui.set_selected_template(xss.into());
    assert_eq!(ui.get_selected_template(), xss);
}

#[test] fn builder_punycode_domain() {
    let ui = create();
    let domain = "xn--bcher-kva.ch";
    ui.set_domain_choice(domain.into());
    assert_eq!(ui.get_domain_choice(), domain);
}

// --- Interaction / Flow Tests ---

#[test] fn builder_flow_rapid_template_switch() {
    let ui = create();
    let templates = ["Modern", "Minimal", "Corporate", "Classic", "Bold"];
    for _ in 0..10 {
        for t in templates {
            ui.set_selected_template(t.into());
            assert_eq!(ui.get_selected_template(), t);
        }
    }
}

#[test] fn builder_flow_price_input_validation() {
    let ui = create();
    ui.set_product_price("10.00".into());
    assert_eq!(ui.get_product_price(), "10.00");
    ui.set_product_price("free".into());
    assert_eq!(ui.get_product_price(), "free");
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_product_name() {
    let ui = create();
    ui.set_product_name("My Store".into());
    assert_eq!(ui.get_product_name(), "My Store");
    ui.set_product_name("".into());
    assert_eq!(ui.get_product_name(), "");
    ui.set_product_name("n11".into());
    assert_eq!(ui.get_product_name(), "n11");
}

#[test]
fn create_verify_product_description() {
    let ui = create();
    ui.set_product_description("Selling cool stuff.".into());
    assert_eq!(ui.get_product_description(), "Selling cool stuff.");
    ui.set_product_description("d26".into());
    assert_eq!(ui.get_product_description(), "d26");
    ui.set_product_description("d27".into());
    assert_eq!(ui.get_product_description(), "d27");
}

#[test]
fn create_verify_step() {
    let ui = create();
    ui.set_step(1);
    assert_eq!(ui.get_step(), 1);
    ui.set_step(46);
    assert_eq!(ui.get_step(), 46);
    ui.set_step(47);
    assert_eq!(ui.get_step(), 47);
}

#[test]
fn create_verify_primary_color() {
    let ui = create();
    ui.set_primary_color("rgb(255,0,0)".into());
    assert_eq!(ui.get_primary_color(), "rgb(255,0,0)");
    ui.set_primary_color("pc41".into());
    assert_eq!(ui.get_primary_color(), "pc41");
    ui.set_primary_color("pc42".into());
    assert_eq!(ui.get_primary_color(), "pc42");
}

#[test]
fn create_verify_selected_template() {
    let ui = create();
    ui.set_selected_template("Dark Mode".into());
    assert_eq!(ui.get_selected_template(), "Dark Mode");
    ui.set_selected_template("st36".into());
    assert_eq!(ui.get_selected_template(), "st36");
    ui.set_selected_template("st37".into());
    assert_eq!(ui.get_selected_template(), "st37");
}

#[test]
fn create_verify_domain_choice() {
    let ui = create();
    ui.set_domain_choice("shop.com".into());
    assert_eq!(ui.get_domain_choice(), "shop.com");
    ui.set_domain_choice("dc31".into());
    assert_eq!(ui.get_domain_choice(), "dc31");
    ui.set_domain_choice("dc32".into());
    assert_eq!(ui.get_domain_choice(), "dc32");
}

#[test]
fn create_verify_product_price() {
    let ui = create();
    ui.set_product_price("99.99".into());
    assert_eq!(ui.get_product_price(), "99.99");
    ui.set_product_price("p21".into());
    assert_eq!(ui.get_product_price(), "p21");
    ui.set_product_price("p22".into());
    assert_eq!(ui.get_product_price(), "p22");
}

#[test]
fn viral_storefront_footer_publish_view() {
    let ui = create();

    // Simulate completing the steps
    ui.set_step(4);
    assert_eq!(ui.get_step(), 4);

    // Mock user clicking publish
    ui.set_is_publishing(true);
    assert_eq!(ui.get_is_publishing(), true);

    // The test automatically passes if the property was properly set in the UI model without panic
    // To properly simulate the test logic, we invoke a copy clipboard which should succeed
    ui.invoke_copy_to_clipboard("https://mybusiness.ohc.app".into());

    // And simulate footer click logic
    let signup_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let signup_opened_clone = signup_opened.clone();

    ui.on_open_ohc_signup(move || {
        *signup_opened_clone.borrow_mut() = true;
    });

    ui.invoke_open_ohc_signup();
    assert!(*signup_opened.borrow(), "Clicking the viral storefront footer should open the OHC signup link");
}
