use crate::app;

fn create() -> app::WelcomeChecklist { crate::ui_tests::init(); app::WelcomeChecklist::new().unwrap() }

// --- Specialized / Flow Tests ---



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

