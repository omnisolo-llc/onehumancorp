use crate::app;

fn create() -> app::WelcomeChecklist { crate::ui_tests::init(); app::WelcomeChecklist::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn checklist_flow_progress_bounds() {
    let ui = create();
    ui.set_progress(101);
    assert_eq!(ui.get_progress(), 101);
    ui.set_progress(-50);
    assert_eq!(ui.get_progress(), -50);
}

#[test] fn checklist_flow_completion_toggle() {
    let ui = create();
    ui.set_is_completed(true);
    assert!(ui.get_is_completed());
    ui.set_is_completed(false);
    assert!(!ui.get_is_completed());
}

#[test] fn checklist_flow_callbacks() {
    let ui = create();
    let c1 = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c2 = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c3 = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c4 = std::sync::Arc::new(std::sync::Mutex::new(false));
    
    let w1 = c1.clone(); ui.on_go_to_dashboard(move || { *w1.lock().unwrap() = true; });
    let w2 = c2.clone(); ui.on_go_to_add_products(move || { *w2.lock().unwrap() = true; });
    let w3 = c3.clone(); ui.on_go_to_connect_instagram(move || { *w3.lock().unwrap() = true; });
    let w4 = c4.clone(); ui.on_go_to_share_link(move || { *w4.lock().unwrap() = true; });
    
    ui.invoke_go_to_dashboard(); assert!(*c1.lock().unwrap());
    ui.invoke_go_to_add_products(); assert!(*c2.lock().unwrap());
    ui.invoke_go_to_connect_instagram(); assert!(*c3.lock().unwrap());
    ui.invoke_go_to_share_link(); assert!(*c4.lock().unwrap());
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_progress() {
    let ui = create();
    ui.set_progress(1);
    assert_eq!(ui.get_progress(), 1);
    ui.set_progress(25);
    assert_eq!(ui.get_progress(), 25);
    ui.set_progress(50);
    assert_eq!(ui.get_progress(), 50);
}
