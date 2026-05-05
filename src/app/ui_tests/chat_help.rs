use crate::app;

fn create() -> app::AiHelpChat { crate::ui_tests::init(); app::AiHelpChat::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn chat_help_flow_send_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_send_message(move || { *c.borrow_mut() = true; });
    ui.invoke_send_message();
    assert!(*called.borrow());
}

#[test] fn chat_help_xss_input() {
    let ui = create();
    let xss = "<script>alert('chat_help')</script>";
    ui.set_user_input(xss.into());
    assert_eq!(ui.get_user_input(), xss);
}

#[test] fn chat_help_injection_input() {
    let ui = create();
    let inj = "Help'); DROP TABLE history; --";
    ui.set_user_input(inj.into());
    assert_eq!(ui.get_user_input(), inj);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_user_input() {
    let ui = create();
    ui.set_user_input("How do I add a product?".into());
    assert_eq!(ui.get_user_input(), "How do I add a product?");
    ui.set_user_input("Can I use Apple Pay?".into());
    assert_eq!(ui.get_user_input(), "Can I use Apple Pay?");
    ui.set_user_input("What is an AI helper?".into());
    assert_eq!(ui.get_user_input(), "What is an AI helper?");
}
