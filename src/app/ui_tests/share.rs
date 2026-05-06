use crate::app;

fn create() -> app::BusinessShare { crate::ui_tests::init(); app::BusinessShare::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn share_xss_name() {
    let ui = create();
    let xss = "<script>alert('share')</script>";
    ui.set_business_name(xss.into());
    assert_eq!(ui.get_business_name(), xss);
}

#[test] fn share_injection_tagline() {
    let ui = create();
    let inj = "Best Store'); DROP TABLE stores; --";
    ui.set_business_tagline(inj.into());
    assert_eq!(ui.get_business_tagline(), inj);
}

#[test] fn share_long_link() {
    let ui = create();
    let long = "ohc://share?b=".to_string() + &"f".repeat(1000);
    ui.set_share_link(long.clone().into());
    assert_eq!(ui.get_share_link(), long);
}

// --- Interaction / Flow Tests ---

#[test] fn share_flow_copy_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_copy_link(move || { *c.borrow_mut() = true; });
    ui.invoke_copy_link();
    assert!(*called.borrow());
}

#[test] fn share_flow_insta_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_share_to_instagram(move || { *c.borrow_mut() = true; });
    ui.invoke_share_to_instagram();
    assert!(*called.borrow());
}

#[test] fn share_flow_x_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_share_to_x(move || { *c.borrow_mut() = true; });
    ui.invoke_share_to_x();
    assert!(*called.borrow());
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_business_name() {
    let ui = create();
    ui.set_business_name("Alpha Store".into());
    assert_eq!(ui.get_business_name(), "Alpha Store");
    ui.set_business_name("s11".into());
    assert_eq!(ui.get_business_name(), "s11");
    ui.set_business_name("s12".into());
    assert_eq!(ui.get_business_name(), "s12");
}

#[test]
fn create_verify_business_tagline() {
    let ui = create();
    ui.set_business_tagline("Quality First".into());
    assert_eq!(ui.get_business_tagline(), "Quality First");
    ui.set_business_tagline("t21".into());
    assert_eq!(ui.get_business_tagline(), "t21");
    ui.set_business_tagline("t22".into());
    assert_eq!(ui.get_business_tagline(), "t22");
}

#[test]
fn create_verify_share_link() {
    let ui = create();
    ui.set_share_link("https://link.com".into());
    assert_eq!(ui.get_share_link(), "https://link.com");
    ui.set_share_link("l41".into());
    assert_eq!(ui.get_share_link(), "l41");
    ui.set_share_link("l42".into());
    assert_eq!(ui.get_share_link(), "l42");
}

#[test] fn share_flow_whatsapp_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_share_to_whatsapp(move || { *c.borrow_mut() = true; });
    ui.invoke_share_to_whatsapp();
    assert!(*called.borrow());
}
