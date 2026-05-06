use slint::{ComponentHandle, SharedString, Model};

#[test]
fn test_scribe_help_center_content() {
    crate::ui_tests::init();
    let ui = crate::app::HelpCenter::new().unwrap();
    let articles = ui.get_articles();
    assert!(articles.row_count() > 0, "Help Center must have articles");

    // Test that the topics from the prompt are present
    let mut found_getting_started = false;
    let mut found_my_store = false;
    let mut found_payments = false;
    let mut found_ai_helpers = false;
    let mut found_marketing = false;
    let mut found_account = false;

    for i in 0..articles.row_count() {
        let article = articles.row_data(i).unwrap();
        match article.category.as_str() {
            "Getting Started" => found_getting_started = true,
            "My Store" => found_my_store = true,
            "Payments & Billing" => found_payments = true,
            "AI Helpers" => found_ai_helpers = true,
            "Marketing" => found_marketing = true,
            "Account & Billing" => found_account = true,
            _ => {}
        }
    }

    assert!(found_getting_started, "Missing Getting Started");
    assert!(found_my_store, "Missing My Store");
    assert!(found_payments, "Missing Payments & Billing");
    assert!(found_ai_helpers, "Missing AI Helpers");
    assert!(found_marketing, "Missing Marketing");
    assert!(found_account, "Missing Account & Billing");
}

#[test]
fn test_scribe_tooltip_registry() {
    crate::ui_tests::init();
    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    dashboard_ui.global::<crate::app::TooltipRegistry>().on_request_tooltip_text(|id| {
        static TOOLTIPS: std::sync::OnceLock<std::collections::HashMap<String, String>> = std::sync::OnceLock::new();
        let tooltips = TOOLTIPS.get_or_init(|| serde_json::from_str(include_str!("../tooltips.json")).unwrap_or_default());
        tooltips.get(id.as_str()).cloned().unwrap_or_default().into()
    });

    let tr = dashboard_ui.global::<crate::app::TooltipRegistry>();

    // Verify some expected tooltips
    tr.invoke_show_tooltip("help_center".into(), 0.0, 0.0);
    assert!(tr.get_is_visible(), "Tooltip should be visible");
    assert_eq!(tr.get_active_text(), SharedString::from("Find answers and how-to guides."));

    tr.invoke_hide_tooltip();
    assert!(!tr.get_is_visible(), "Tooltip should hide");
}

#[test]
fn test_scribe_interactive_walkthrough() {
    crate::ui_tests::init();
    let ui = crate::app::InteractiveWalkthrough::new().unwrap();

    // Verify the steps
    ui.set_current_step(0);
    assert_eq!(ui.get_current_step(), 0);

    ui.set_current_step(1);
    assert_eq!(ui.get_current_step(), 1);

    ui.set_current_step(2);
    assert_eq!(ui.get_current_step(), 2);

    ui.set_current_step(3);
    assert_eq!(ui.get_current_step(), 3);
}

#[test]
fn test_scribe_ai_help_chat() {
    crate::ui_tests::init();
    let ui = crate::app::AiHelpChat::new().unwrap();
    let messages = ui.get_messages();

    assert!(messages.row_count() > 0, "AI Chat must have initial messages");
    let initial_msg = messages.row_data(0).unwrap();
    assert_eq!(initial_msg.sender, "AI");

    let link_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let link_opened_clone = link_opened.clone();
    ui.on_open_article(move |_link| {
        *link_opened_clone.borrow_mut() = true;
    });

    ui.invoke_open_article("some_link".into());
    assert!(*link_opened.borrow(), "Article link opening should trigger callback");

    ui.set_user_input("How do I add a product?".into());
    assert_eq!(ui.get_user_input(), SharedString::from("How do I add a product?"));

    let sent_message = std::rc::Rc::new(std::cell::RefCell::new(false));
    let sent_message_clone = sent_message.clone();
    ui.on_send_message(move || {
        *sent_message_clone.borrow_mut() = true;
    });

    ui.invoke_send_message();
    assert!(*sent_message.borrow(), "Send message callback should be triggered");
}

#[test]
fn test_scribe_video_tutorials() {
    crate::ui_tests::init();
    let ui = crate::app::VideoTutorials::new().unwrap();

    ui.set_is_playing(true);
    assert!(ui.get_is_playing(), "Video should be marked as playing");

    ui.set_selected_video_title("Setting up your store".into());
    assert_eq!(ui.get_selected_video_title(), SharedString::from("Setting up your store"));

    let custom_videos = vec![
        crate::app::VideoMetadata {
            title: "Test Video".into(),
            description: "Test Desc".into(),
            duration_sec: 120,
            url: "http://example.com/video".into(),
            thumbnail_url: "http://example.com/thumb".into(),
        }
    ];
    let videos_model = slint::ModelRc::new(slint::VecModel::from(custom_videos));
    ui.set_videos(videos_model.into());

    let videos = ui.get_videos();
    assert_eq!(videos.row_count(), 1, "Should have 1 video in the model");
    let vid = videos.row_data(0).unwrap();
    assert_eq!(vid.title, "Test Video");
}

#[test]
fn test_scribe_api_docs() {
    crate::ui_tests::init();
    let ui = crate::app::ApiDocs::new().unwrap();

    assert!(!ui.get_is_advanced(), "Advanced docs hidden by default");
    ui.set_is_advanced(true);
    assert!(ui.get_is_advanced(), "Advanced docs visible when toggled");

    let custom_endpoints = vec![
        crate::app::ApiEndpoint {
            method: "GET".into(),
            path: "/api/test".into(),
            description: "Test endpoint".into(),
        }
    ];
    let endpoints_model = slint::ModelRc::new(slint::VecModel::from(custom_endpoints));
    ui.set_endpoints(endpoints_model.into());

    assert_eq!(ui.get_endpoints().row_count(), 1);

    let endpoint_tested = std::rc::Rc::new(std::cell::RefCell::new(false));
    let endpoint_tested_clone = endpoint_tested.clone();
    ui.on_test_endpoint(move |_| {
        *endpoint_tested_clone.borrow_mut() = true;
    });

    ui.invoke_test_endpoint("/api/v1/store".into());
    assert!(*endpoint_tested.borrow(), "Endpoint test should trigger callback");

    ui.set_api_response("{\"status\": \"success\"}".into());
    assert_eq!(ui.get_api_response(), SharedString::from("{\"status\": \"success\"}"));

    ui.set_active_endpoint("/api/v1/store".into());
    assert_eq!(ui.get_active_endpoint(), SharedString::from("/api/v1/store"));
}

#[test]
fn test_scribe_release_notes() {
    crate::ui_tests::init();
    let ui = crate::app::ReleaseNotes::new().unwrap();

    assert_eq!(ui.get_current_version(), SharedString::from("v0.3.4"));

    ui.set_show_latest_only(true);
    assert!(ui.get_show_latest_only(), "Show latest only toggle should work");
}
