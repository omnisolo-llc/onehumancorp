use slint::Model;
use crate::app;

#[test]
fn test_scribe_dashboard_quick_actions_hint_tooltip() {
    crate::ui_tests::init();
    let dashboard_ui = app::Dashboard::new().unwrap();

    // Default is false
    assert_eq!(dashboard_ui.get_show_quick_actions_hint(), false);

    // We can't click easily, but we can verify the property exists and updates.
    dashboard_ui.set_show_quick_actions_hint(true);
    assert_eq!(dashboard_ui.get_show_quick_actions_hint(), true);
}

#[test]
fn test_scribe_video_tutorials_url() {
    crate::ui_tests::init();
    let ui = app::VideoTutorials::new().unwrap();
    let videos = ui.get_videos();

    // We can add a video and check its URL.
    let new_video = app::VideoMetadata {
        title: "Test Video".into(),
        description: "Test".into(),
        duration_sec: 120,
        url: "https://test.com/video.mp4".into(),
        thumbnail_url: "".into(),
    };

    let mut vec = Vec::new();
    for i in 0..videos.row_count() {
        vec.push(videos.row_data(i).unwrap());
    }
    vec.push(new_video);

    ui.set_videos(slint::ModelRc::new(slint::VecModel::from(vec)));

    let updated_videos = ui.get_videos();
    assert_eq!(updated_videos.row_data(updated_videos.row_count() - 1).unwrap().url, "https://test.com/video.mp4");
}

#[test]
fn test_scribe_dashboard_comprehensive_functionality() {
    crate::ui_tests::init();
    let dashboard = app::ScribeFeatureDashboard::new().unwrap();

    let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let opened_clone = opened.clone();
    dashboard.on_open_help_center(move || { *opened_clone.borrow_mut() = true; });
    dashboard.invoke_open_help_center();
    assert!(*opened.borrow(), "Help center must open");

    let chat_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let chat_clone = chat_opened.clone();
    dashboard.on_open_ai_chat(move || { *chat_clone.borrow_mut() = true; });
    dashboard.invoke_open_ai_chat();
    assert!(*chat_opened.borrow(), "AI Chat must open");

    let walkthrough_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let wt_clone = walkthrough_opened.clone();
    dashboard.on_open_walkthrough(move || { *wt_clone.borrow_mut() = true; });
    dashboard.invoke_open_walkthrough();
    assert!(*walkthrough_opened.borrow(), "Walkthrough must open");

    let videos_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let videos_clone = videos_opened.clone();
    dashboard.on_open_video_tutorials(move || { *videos_clone.borrow_mut() = true; });
    dashboard.invoke_open_video_tutorials();
    assert!(*videos_opened.borrow(), "Videos must open");

    let api_docs_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let api_docs_clone = api_docs_opened.clone();
    dashboard.on_open_api_docs(move || { *api_docs_clone.borrow_mut() = true; });
    dashboard.invoke_open_api_docs();
    assert!(*api_docs_opened.borrow(), "API Docs must open");

    let notes_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let notes_clone = notes_opened.clone();
    dashboard.on_open_release_notes(move || { *notes_clone.borrow_mut() = true; });
    dashboard.invoke_open_release_notes();
    assert!(*notes_opened.borrow(), "Release notes must open");
}

#[test]
fn test_scribe_help_center_search_properties() {
    crate::ui_tests::init();
    let ui = app::HelpCenter::new().unwrap();
    assert_eq!(ui.get_search_query(), slint::SharedString::from(""));
    ui.set_search_query("test".into());
    assert_eq!(ui.get_search_query(), slint::SharedString::from("test"));
}

#[test]
fn test_scribe_api_docs_response_property() {
    crate::ui_tests::init();
    let ui = app::ApiDocs::new().unwrap();
    assert_eq!(ui.get_api_response(), slint::SharedString::from(""));
    ui.set_api_response("{ \"status\": \"ok\" }".into());
    assert_eq!(ui.get_api_response(), slint::SharedString::from("{ \"status\": \"ok\" }"));
    assert_eq!(ui.get_active_endpoint(), slint::SharedString::from(""));
    ui.set_active_endpoint("/v1/test".into());
    assert_eq!(ui.get_active_endpoint(), slint::SharedString::from("/v1/test"));
}

#[test]
fn test_scribe_ai_help_chat_user_input() {
    crate::ui_tests::init();
    let ui = app::AiHelpChat::new().unwrap();
    assert_eq!(ui.get_user_input(), slint::SharedString::from(""));
    ui.set_user_input("How do I add a product?".into());
    assert_eq!(ui.get_user_input(), slint::SharedString::from("How do I add a product?"));

    let sent = std::rc::Rc::new(std::cell::RefCell::new(false));
    let sent_clone = sent.clone();
    ui.on_send_message(move || { *sent_clone.borrow_mut() = true; });
    ui.invoke_send_message();
    assert!(*sent.borrow(), "Send message must trigger");
}
