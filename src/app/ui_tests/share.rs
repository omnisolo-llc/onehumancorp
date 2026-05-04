use crate::app;
use slint::ComponentHandle;

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

#[test]
fn e2e_business_share_full_flow() {
    crate::ui_tests::init();

    // 1. Initialize Login
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

    // 2. Dashboard
    let dashboard_ui = app::Dashboard::new().unwrap();

    // 3. Setup Share Flow using the shared helper
    let business_share_ui = app::BusinessShare::new().unwrap();
    crate::configure_business_share_ui(&business_share_ui);

    let share_store_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let share_store_called_clone = share_store_called.clone();

    dashboard_ui.on_action_share_store({
        move || {
            *share_store_called_clone.borrow_mut() = true;
            let _ = business_share_ui.show();
        }
    });

    // 4. Trigger the flow
    dashboard_ui.invoke_action_share_store();
    assert!(*share_store_called.borrow(), "Share Store should be invoked from Dashboard");
}
