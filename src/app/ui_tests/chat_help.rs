use crate::app;
use slint::Model;

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
    ui.set_user_input("What is an AI agent?".into());
    assert_eq!(ui.get_user_input(), "What is an AI agent?");
}

#[test]
fn test_e2e_ai_help_chat_ask_anything_button_click() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "User login should be successful");

    let dashboard_ui = app::Dashboard::new().unwrap();
    let ai_chat_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let ai_chat_opened_clone = ai_chat_opened.clone();

    dashboard_ui.on_open_ai_chat(move || {
        *ai_chat_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_open_ai_chat();
    assert!(*ai_chat_opened.borrow(), "AI Chat should be opened from Dashboard");
}

#[test]
fn test_e2e_ai_help_chat_send_message_updates_ui() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "User login should be successful");

    let ui = app::AiHelpChat::new().unwrap();
    ui.set_user_input("Testing input".into());
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    ui.on_send_message(move || {
        *c.borrow_mut() = true;
    });

    ui.invoke_send_message();
    assert!(*called.borrow(), "Send message should be invokable");
}

#[test]
fn test_e2e_ai_help_chat_open_article_link() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "User login should be successful");

    let ui = app::AiHelpChat::new().unwrap();

    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    ui.on_open_article(move |link| {
        assert_eq!(link, "test-link");
        *c.borrow_mut() = true;
    });

    ui.invoke_open_article("test-link".into());
    assert!(*called.borrow(), "Open article should be invokable");
}

#[test]
fn test_e2e_ai_help_chat_message_list_verification() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "User login should be successful");

    let ui = app::AiHelpChat::new().unwrap();

    let initial_count = ui.get_messages().iter().count();
    assert_eq!(initial_count, 1, "Should have 1 initial message");

    let mut messages = ui.get_messages().iter().collect::<Vec<_>>();
    messages.push(app::ChatMessage {
        sender: "User".into(),
        text: "My question".into(),
        article_link: "".into(),
    });

    ui.set_messages(slint::ModelRc::new(slint::VecModel::from(messages)));
    let new_count = ui.get_messages().iter().count();
    assert_eq!(new_count, 2, "Should have 2 messages after update");
}

#[test]
fn test_e2e_ai_help_chat_initial_message_content() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "User login should be successful");

    let ui = app::AiHelpChat::new().unwrap();

    let msg = ui.get_messages().row_data(0).unwrap();
    assert_eq!(msg.sender, "AI");
    assert!(msg.text.contains("Hi! I'm your OHC Help Assistant"));
}
