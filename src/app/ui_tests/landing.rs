use crate::app;

fn create() -> app::Landing { crate::ui_tests::init(); app::Landing::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn landing_flow_variant_toggle() {
    let ui = create();
    ui.set_is_variant_b(false);
    assert!(!ui.get_is_variant_b());
    ui.set_is_variant_b(true);
    assert!(ui.get_is_variant_b());
}

#[test] fn landing_flow_start_setup_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_start_business_setup(move || { *c.borrow_mut() = true; });
    ui.invoke_start_business_setup();
    assert!(*called.borrow());
}

#[test] fn landing_flow_continue_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_continue_to_dashboard(move || { *c.borrow_mut() = true; });
    ui.invoke_continue_to_dashboard();
    assert!(*called.borrow());
}

#[test] fn landing_flow_download_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_download(move |os| { *c.borrow_mut() = os.to_string(); });
    
    ui.invoke_download("Mac".into());
    assert_eq!(*called.borrow(), "Mac");
    ui.invoke_download("Linux".into());
    assert_eq!(*called.borrow(), "Linux");
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
