use slint::{ComponentHandle, SharedString, Model};

#[test]
fn test_help_center_edge_cases() {
    crate::ui_tests::init();
    let ui = crate::app::HelpCenter::new().unwrap();

    // Verify initial search query is empty
    assert_eq!(ui.get_search_query(), SharedString::from(""));

    // Verify searching triggers the execution callback
    let search_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let search_triggered_clone = search_triggered.clone();
    ui.on_execute_search(move || {
        *search_triggered_clone.borrow_mut() = true;
    });

    ui.set_search_query("Advanced".into());
    assert_eq!(ui.get_search_query(), SharedString::from("Advanced"));
    ui.invoke_execute_search();
    assert!(*search_triggered.borrow());

    let articles = ui.get_articles();
    assert!(articles.row_count() >= 7, "Should have at least 7 predefined articles");

    // Let's verify the first article
    let first = articles.row_data(0).unwrap();
    assert_eq!(first.category, SharedString::from("Getting Started"));
    assert_eq!(first.title, SharedString::from("Set up your store in 5 minutes"));
}

#[test]
fn test_ai_help_chat_edge_cases() {
    crate::ui_tests::init();
    let ui = crate::app::AiHelpChat::new().unwrap();

    let initial_msg = ui.get_messages().row_data(0).unwrap();
    assert_eq!(initial_msg.sender, SharedString::from("AI"));
    assert_eq!(initial_msg.article_link, SharedString::from(""));

    let send_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let send_triggered_clone = send_triggered.clone();
    ui.on_send_message(move || {
        *send_triggered_clone.borrow_mut() = true;
    });

    ui.set_user_input("Help me test!".into());
    assert_eq!(ui.get_user_input(), SharedString::from("Help me test!"));
    ui.invoke_send_message();
    assert!(*send_triggered.borrow());
}

#[test]
fn test_interactive_walkthrough_edge_cases() {
    crate::ui_tests::init();
    let ui = crate::app::InteractiveWalkthrough::new().unwrap();

    // Step out of bounds / edge state tests
    ui.set_current_step(10);
    assert_eq!(ui.get_current_step(), 10);

    ui.set_current_step(-1);
    assert_eq!(ui.get_current_step(), -1);
}

#[test]
fn test_video_tutorials_metadata() {
    crate::ui_tests::init();
    let ui = crate::app::VideoTutorials::new().unwrap();

    let videos = ui.get_videos();
    assert!(videos.row_count() >= 10, "Should have 10 top tutorials per requirement");

    let first = videos.row_data(0).unwrap();
    assert_eq!(first.title, SharedString::from("How to add your first product"));
    assert!(first.duration_sec <= 90, "Tutorials should be short <90s");

    let last = videos.row_data(videos.row_count() - 1).unwrap();
    assert!(last.duration_sec <= 90, "Tutorials should be short <90s");
}

#[test]
fn test_api_docs_edge_cases() {
    crate::ui_tests::init();
    let ui = crate::app::ApiDocs::new().unwrap();

    // Default should be false as verified
    assert!(!ui.get_is_advanced());
    ui.set_is_advanced(true);
    assert!(ui.get_is_advanced());

    ui.set_api_response("{\"status\": \"ok\"}".into());
    assert_eq!(ui.get_api_response(), SharedString::from("{\"status\": \"ok\"}"));

    ui.set_active_endpoint("/v1/test".into());
    assert_eq!(ui.get_active_endpoint(), SharedString::from("/v1/test"));
}

#[test]
fn test_release_notes_edge_cases() {
    crate::ui_tests::init();
    let ui = crate::app::ReleaseNotes::new().unwrap();

    assert_eq!(ui.get_current_version(), SharedString::from("v0.4.32"));

    ui.set_current_version("v1.0.0".into());
    assert_eq!(ui.get_current_version(), SharedString::from("v1.0.0"));

    assert!(!ui.get_show_latest_only());
    ui.set_show_latest_only(true);
    assert!(ui.get_show_latest_only());
}
