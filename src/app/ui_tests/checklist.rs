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
    let c1 = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c2 = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c3 = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c4 = std::rc::Rc::new(std::cell::RefCell::new(false));
    
    let w1 = c1.clone(); ui.on_go_to_dashboard(move || { *w1.borrow_mut() = true; });
    let w2 = c2.clone(); ui.on_go_to_add_products(move || { *w2.borrow_mut() = true; });
    let w3 = c3.clone(); ui.on_go_to_connect_instagram(move || { *w3.borrow_mut() = true; });
    let w4 = c4.clone(); ui.on_go_to_share_link(move || { *w4.borrow_mut() = true; });
    
    ui.invoke_go_to_dashboard(); assert!(*c1.borrow());
    ui.invoke_go_to_add_products(); assert!(*c2.borrow());
    ui.invoke_go_to_connect_instagram(); assert!(*c3.borrow());
    ui.invoke_go_to_share_link(); assert!(*c4.borrow());
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

#[test]
fn test_e2e_welcome_checklist_verification() {
    let ui = create();
    ui.set_progress(0);
    assert_eq!(ui.get_progress(), 0);

    // Simulate progressing to 100% completion
    ui.set_progress(100);
    ui.set_is_completed(true);
    assert_eq!(ui.get_progress(), 100);
    assert!(ui.get_is_completed());

    // Test callbacks for clicking links
    let clicked_products = std::rc::Rc::new(std::cell::RefCell::new(false));
    let cp = clicked_products.clone();
    ui.on_go_to_add_products(move || {
        *cp.borrow_mut() = true;
    });
    ui.invoke_go_to_add_products();
    assert!(*clicked_products.borrow());

    let clicked_insta = std::rc::Rc::new(std::cell::RefCell::new(false));
    let ci = clicked_insta.clone();
    ui.on_go_to_connect_instagram(move || {
        *ci.borrow_mut() = true;
    });
    ui.invoke_go_to_connect_instagram();
    assert!(*clicked_insta.borrow());
}
