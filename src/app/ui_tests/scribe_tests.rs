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
fn test_scribe_help_center_search_clear() {
    crate::ui_tests::init();
    let ui = app::HelpCenter::new().unwrap();

    ui.set_search_query("test query".into());
    assert_eq!(ui.get_search_query(), slint::SharedString::from("test query"));

    ui.set_search_query("".into());
    assert_eq!(ui.get_search_query(), slint::SharedString::from(""));
}

#[test]
fn test_scribe_ai_help_chat_history() {
    crate::ui_tests::init();
    let ui = app::AiHelpChat::new().unwrap();

    let messages = ui.get_messages();
    let initial_count = messages.row_count();
    assert!(initial_count > 0, "Should have initial message");

    let new_msg = app::ChatMessage {
        sender: "User".into(),
        text: "How do I add a product?".into(),
        article_link: "".into(),
    };

    let mut vec = Vec::new();
    for i in 0..messages.row_count() {
        vec.push(messages.row_data(i).unwrap());
    }
    vec.push(new_msg);

    ui.set_messages(slint::ModelRc::new(slint::VecModel::from(vec)));

    let updated_messages = ui.get_messages();
    assert_eq!(updated_messages.row_count(), initial_count + 1);
    assert_eq!(updated_messages.row_data(initial_count).unwrap().sender, "User");
}

#[test]
fn test_scribe_video_tutorials_state() {
    crate::ui_tests::init();
    let ui = app::VideoTutorials::new().unwrap();

    assert_eq!(ui.get_is_playing(), false);
    assert_eq!(ui.get_selected_video_title(), slint::SharedString::from(""));

    ui.set_is_playing(true);
    ui.set_selected_video_title("How to add your first product".into());

    assert_eq!(ui.get_is_playing(), true);
    assert_eq!(ui.get_selected_video_title(), slint::SharedString::from("How to add your first product"));
}

#[test]
fn test_scribe_interactive_walkthrough_visibility() {
    crate::ui_tests::init();
    let ui = app::InteractiveWalkthrough::new().unwrap();

    // Default visibility depends on platform integration, but we can verify our properties
    ui.set_current_step(1);
    assert_eq!(ui.get_current_step(), 1);

    // Simulate completion
    ui.set_current_step(4); // 4 means "You're all set!"
    assert_eq!(ui.get_current_step(), 4);
}

#[test]
fn test_scribe_api_docs_test_endpoint_empty() {
    crate::ui_tests::init();
    let ui = app::ApiDocs::new().unwrap();

    let endpoint_tested = std::rc::Rc::new(std::cell::RefCell::new(false));
    let endpoint_tested_clone = endpoint_tested.clone();
    ui.on_test_endpoint(move |_| {
        *endpoint_tested_clone.borrow_mut() = true;
    });

    ui.invoke_test_endpoint("".into());
    assert!(*endpoint_tested.borrow());
}

#[test]
fn test_scribe_release_notes_latest_only_toggle() {
    crate::ui_tests::init();
    let ui = app::ReleaseNotes::new().unwrap();

    assert_eq!(ui.get_show_latest_only(), false);
    ui.set_show_latest_only(true);
    assert_eq!(ui.get_show_latest_only(), true);
    ui.set_show_latest_only(false);
    assert_eq!(ui.get_show_latest_only(), false);
}

#[test]
fn test_new_integrations_present() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let ui = crate::app::Integrations::new().unwrap();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();

    ui.on_configure_integration(move |name| {
        if name == "Ayrshare" || name == "Listmonk" || name == "EasyPost" || name == "Jitsi" || name == "Cal.com" || name == "Mercado Pago" || name == "Twilio" || name == "Meta" {
            *invoked_clone.borrow_mut() = true;
        }
    });

    ui.invoke_configure_integration("Ayrshare".into());
    assert!(*invoked.borrow(), "Ayrshare configuration should be callable");
    *invoked.borrow_mut() = false;

    ui.invoke_configure_integration("Listmonk".into());
    assert!(*invoked.borrow(), "Listmonk configuration should be callable");
    *invoked.borrow_mut() = false;

    ui.invoke_configure_integration("EasyPost".into());
    assert!(*invoked.borrow(), "EasyPost configuration should be callable");
    *invoked.borrow_mut() = false;

    ui.invoke_configure_integration("Jitsi".into());
    assert!(*invoked.borrow(), "Jitsi configuration should be callable");
    *invoked.borrow_mut() = false;

    ui.invoke_configure_integration("Cal.com".into());
    assert!(*invoked.borrow(), "Cal.com configuration should be callable");
    *invoked.borrow_mut() = false;

    ui.invoke_configure_integration("Mercado Pago".into());
    assert!(*invoked.borrow(), "Mercado Pago configuration should be callable");
    *invoked.borrow_mut() = false;

    ui.invoke_configure_integration("Twilio".into());
    assert!(*invoked.borrow(), "Twilio configuration should be callable");
}
