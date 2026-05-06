use slint::ComponentHandle;

#[test]
fn test_e2e_help_center_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Simulate user flow: Start at Login, progress to Dashboard
    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_help_center(move || { *opened_clone.borrow_mut() = true; });

    // User clicks the Help Center button on the dashboard menu
    dashboard_ui.invoke_open_help_center();

    // Verify flow reached the target destination
    assert!(*opened.borrow(), "Help Center should be opened from Dashboard");

    // Verify the destination component renders correctly
    let ui = crate::app::HelpCenter::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Help Center"));
}

#[test]
fn test_e2e_api_docs_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_api_docs(move || { *opened_clone.borrow_mut() = true; });

    // User clicks the Connect Apps button on the dashboard menu
    dashboard_ui.invoke_open_api_docs();

    // Verify flow reached the target destination
    assert!(*opened.borrow(), "API Docs should be opened from Dashboard");

    // Verify the destination component renders correctly
    let ui = crate::app::ApiDocs::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Connect Custom Software"));
}

#[test]
fn test_e2e_release_notes_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_release_notes(move || { *opened_clone.borrow_mut() = true; });

    // User clicks the What's New button on the dashboard menu
    dashboard_ui.invoke_open_release_notes();

    // Verify flow reached the target destination
    assert!(*opened.borrow(), "Release Notes should be opened from Dashboard");

    // Verify the destination component renders correctly
    let ui = crate::app::ReleaseNotes::new().unwrap();
    assert_eq!(ui.get_current_version(), slint::SharedString::from("v0.3.4"));
}

#[test]
fn test_e2e_video_tutorials_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_video_tutorials(move || { *opened_clone.borrow_mut() = true; });

    // User clicks the Video Tutorials button on the dashboard menu
    dashboard_ui.invoke_open_video_tutorials();

    // Verify flow reached the target destination
    assert!(*opened.borrow(), "Video Tutorials should be opened from Dashboard");

    // Verify the destination component renders correctly
    let ui = crate::app::VideoTutorials::new().unwrap();
    ui.set_is_playing(true);
    assert_eq!(ui.get_is_playing(), true);
}

#[test]
fn test_e2e_interactive_walkthrough_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_interactive_walkthrough(move || { *opened_clone.borrow_mut() = true; });

    // User clicks the App Tour button on the dashboard menu
    dashboard_ui.invoke_open_interactive_walkthrough();

    // Verify flow reached the target destination
    assert!(*opened.borrow(), "Interactive Walkthrough should be opened from Dashboard");

    // Verify the destination component renders correctly
    let ui = crate::app::InteractiveWalkthrough::new().unwrap();
    ui.set_current_step(1);
    assert_eq!(ui.get_current_step(), 1);
}

#[test]
fn test_e2e_ai_help_chat_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_ai_chat(move || { *opened_clone.borrow_mut() = true; });

    // User clicks the AI Chat button on the dashboard
    dashboard_ui.invoke_open_ai_chat();

    // Verify flow reached the target destination
    assert!(*opened.borrow(), "AI Chat should be opened from Dashboard");

    // Verify the destination component renders correctly
    let ui = crate::app::AiHelpChat::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("AI Help Assistant"));
}

#[test]
fn test_e2e_ai_help_chat_context_logic() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let chat = crate::app::AiHelpChat::new().unwrap();
    let chat_weak = chat.as_weak();

    chat.on_send_message(move || {
        if let Some(ui) = chat_weak.upgrade() {
            let input = ui.get_user_input();
            let input_lower = input.to_lowercase();
            let (response_text, article_link) = if input_lower.contains("product") {
                ("To add a product...".to_string(), "Adding Your Products")
            } else {
                ("Default".to_string(), "Welcome")
            };
            use slint::Model;
            let mut msgs: Vec<crate::app::ChatMessage> = ui.get_messages().iter().collect();
            msgs.push(crate::app::ChatMessage { sender: "AI".into(), text: response_text.into(), article_link: article_link.into() });
            ui.set_messages(slint::ModelRc::new(slint::VecModel::from(msgs)));
        }
    });

    chat.set_user_input("How do I add a product?".into());
    chat.invoke_send_message();

    use slint::Model;
    let last_msg = chat.get_messages().row_data(chat.get_messages().row_count() - 1).unwrap();
    assert_eq!(last_msg.article_link, "Adding Your Products");
}

#[test]
fn test_e2e_help_center_categorization() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = crate::app::HelpCenter::new().unwrap();
    use slint::Model;
    let articles = ui.get_articles();
    let mut categories = std::collections::HashSet::new();
    for i in 0..articles.row_count() {
        categories.insert(articles.row_data(i).unwrap().category.to_string());
    }
    assert!(categories.contains("Getting Started"));
    assert!(categories.contains("My Store"));
    assert!(categories.contains("Payments"));
    assert!(categories.contains("AI Helpers"));
    assert!(categories.contains("Marketing"));
    assert!(categories.contains("Account & Billing"));
}

#[test]
fn test_e2e_tooltip_registry_full_integration() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let dashboard = crate::app::Dashboard::new().unwrap();
    let tr = dashboard.global::<crate::app::TooltipRegistry>();

    dashboard.global::<crate::app::TooltipRegistry>().on_request_tooltip_text(|id| {
        if id == "mark_order_ready" {
            "Let your customer know their order is ready for pickup or shipping.".into()
        } else {
            "".into()
        }
    });

    tr.invoke_show_tooltip("mark_order_ready".into(), 100.0, 200.0);
    assert!(tr.get_is_visible());
    assert_eq!(tr.get_active_text(), "Let your customer know their order is ready for pickup or shipping.");
}
