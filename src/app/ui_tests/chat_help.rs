use crate::app;

fn create() -> app::AiHelpChat { crate::ui_tests::init(); app::AiHelpChat::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn chat_help_flow_send_callback() {
    use slint::ComponentHandle;
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    let ui_weak = ui.as_weak();
    ui.on_send_message(move || {
        *c.borrow_mut() = true;
        if let Some(ui) = ui_weak.upgrade() {
            let input = ui.get_user_input();
            if input.trim().is_empty() { return; }
            let mut messages: Vec<slint::SharedString> = slint::Model::iter(&ui.get_chat_history()).collect();
            messages.push(input.clone());
            messages.push(format!("Here is the documentation about '{}': Read the full article →", input).into());
            ui.set_chat_history(slint::ModelRc::new(slint::VecModel::from(messages)));
            ui.set_user_input("".into());
        }
    });

    ui.set_user_input("Hello AI".into());
    ui.invoke_send_message();

    assert!(*called.borrow());
    assert_eq!(ui.get_user_input(), "");
    let history: Vec<slint::SharedString> = slint::Model::iter(&ui.get_chat_history()).collect();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], "Hello AI");
    assert_eq!(history[1], "Here is the documentation about 'Hello AI': Read the full article →");
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
    ui.set_user_input("What is an AI agent?".into());
    assert_eq!(ui.get_user_input(), "What is an AI agent?");
}

#[test] fn chat_help_e2e_multiple_messages() {
    use slint::ComponentHandle;
    let ui = create();
    let ui_weak = ui.as_weak();
    ui.on_send_message(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let input = ui.get_user_input();
            let mut messages: Vec<slint::SharedString> = slint::Model::iter(&ui.get_chat_history()).collect();
            messages.push(input.clone());
            messages.push(format!("Here is the documentation about '{}': Read the full article →", input).into());
            ui.set_chat_history(slint::ModelRc::new(slint::VecModel::from(messages)));
            ui.set_user_input("".into());
        }
    });

    ui.set_user_input("First".into());
    ui.invoke_send_message();
    ui.set_user_input("Second".into());
    ui.invoke_send_message();

    let history: Vec<slint::SharedString> = slint::Model::iter(&ui.get_chat_history()).collect();
    assert_eq!(history.len(), 4);
    assert_eq!(history[2], "Second");
}

#[test] fn chat_help_e2e_empty_input() {
    use slint::ComponentHandle;
    let ui = create();
    let ui_weak = ui.as_weak();
    ui.on_send_message(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let input = ui.get_user_input();
            if input.trim().is_empty() { return; }
            let mut messages: Vec<slint::SharedString> = slint::Model::iter(&ui.get_chat_history()).collect();
            messages.push(input.clone());
            messages.push(format!("Here is the documentation about '{}': Read the full article →", input).into());
            ui.set_chat_history(slint::ModelRc::new(slint::VecModel::from(messages)));
            ui.set_user_input("".into());
        }
    });

    ui.set_user_input("   ".into());
    ui.invoke_send_message();

    let history: Vec<slint::SharedString> = slint::Model::iter(&ui.get_chat_history()).collect();
    assert_eq!(history.len(), 0);
}
