use crate::app;

#[test]
fn test_help_center_search() {
    crate::ui_tests::init();
    let ui = app::HelpCenter::new().unwrap();
    ui.set_search_query("billing".into());
    assert_eq!(ui.get_search_query(), "billing");
}

#[test]
fn test_ai_help_chat_messages() {
    crate::ui_tests::init();
    let ui = app::AiHelpChat::new().unwrap();
    ui.set_user_input("How do I add a product?".into());
    assert_eq!(ui.get_user_input(), "How do I add a product?");
}

#[test]
fn test_interactive_walkthrough_steps() {
    crate::ui_tests::init();
    let ui = app::InteractiveWalkthrough::new().unwrap();
    assert_eq!(ui.get_current_step(), 0);
    ui.set_current_step(1);
    assert_eq!(ui.get_current_step(), 1);
}

#[test]
fn test_video_tutorials_state() {
    crate::ui_tests::init();
    let ui = app::VideoTutorials::new().unwrap();
    assert!(!ui.get_is_playing());
    ui.set_is_playing(true);
    assert!(ui.get_is_playing());
}

#[test]
fn test_release_notes_version() {
    crate::ui_tests::init();
    let ui = app::ReleaseNotes::new().unwrap();
    assert_eq!(ui.get_current_version(), "v0.3.4");
}
