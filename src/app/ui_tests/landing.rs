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
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c = called.clone();
    ui.on_start_business_setup(move || { *c.lock().unwrap() = true; });
    ui.invoke_start_business_setup();
    assert!(*called.lock().unwrap());
}

#[test] fn landing_flow_continue_callback() {
    let ui = create();
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c = called.clone();
    ui.on_continue_to_dashboard(move || { *c.lock().unwrap() = true; });
    ui.invoke_continue_to_dashboard();
    assert!(*called.lock().unwrap());
}

#[test] fn landing_flow_download_callback() {
    let ui = create();
    let called = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let c = called.clone();
    ui.on_download(move |os| { *c.lock().unwrap() = os.to_string(); });
    
    ui.invoke_download("Mac".into());
    assert_eq!(*called.lock().unwrap(), "Mac");
    ui.invoke_download("Linux".into());
    assert_eq!(*called.lock().unwrap(), "Linux");
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
