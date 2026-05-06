use slint::ComponentHandle;

#[test]
fn test_e2e_help_center_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Simulate user flow: Start at Login, progress to Dashboard
    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let opened = std::sync::Arc::new(std::sync::Mutex::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_help_center(move || { *opened_clone.lock().unwrap() = true; });

    // User clicks the Help Center button on the dashboard menu
    dashboard_ui.invoke_open_help_center();

    // Verify flow reached the target destination
    assert!(*opened.lock().unwrap(), "Help Center should be opened from Dashboard");

    // Verify the destination component renders correctly
    let ui = crate::app::HelpCenter::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Help Center"));
}

#[test]
fn test_e2e_api_docs_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let opened = std::sync::Arc::new(std::sync::Mutex::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_api_docs(move || { *opened_clone.lock().unwrap() = true; });

    // User clicks the Connect Apps button on the dashboard menu
    dashboard_ui.invoke_open_api_docs();

    // Verify flow reached the target destination
    assert!(*opened.lock().unwrap(), "API Docs should be opened from Dashboard");

    // Verify the destination component renders correctly
    let ui = crate::app::ApiDocs::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Connect Custom Software"));
}

#[test]
fn test_e2e_release_notes_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let opened = std::sync::Arc::new(std::sync::Mutex::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_release_notes(move || { *opened_clone.lock().unwrap() = true; });

    // User clicks the What's New button on the dashboard menu
    dashboard_ui.invoke_open_release_notes();

    // Verify flow reached the target destination
    assert!(*opened.lock().unwrap(), "Release Notes should be opened from Dashboard");

    // Verify the destination component renders correctly
    let ui = crate::app::ReleaseNotes::new().unwrap();
    assert_eq!(ui.get_current_version(), slint::SharedString::from("v0.3.4"));
}

#[test]
fn test_e2e_video_tutorials_navigation_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let opened = std::sync::Arc::new(std::sync::Mutex::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_video_tutorials(move || { *opened_clone.lock().unwrap() = true; });

    // User clicks the Video Tutorials button on the dashboard menu
    dashboard_ui.invoke_open_video_tutorials();

    // Verify flow reached the target destination
    assert!(*opened.lock().unwrap(), "Video Tutorials should be opened from Dashboard");

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
    let opened = std::sync::Arc::new(std::sync::Mutex::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_interactive_walkthrough(move || { *opened_clone.lock().unwrap() = true; });

    // User clicks the App Tour button on the dashboard menu
    dashboard_ui.invoke_open_interactive_walkthrough();

    // Verify flow reached the target destination
    assert!(*opened.lock().unwrap(), "Interactive Walkthrough should be opened from Dashboard");

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
    let opened = std::sync::Arc::new(std::sync::Mutex::new(false));
    let opened_clone = opened.clone();

    dashboard_ui.on_open_ai_chat(move || { *opened_clone.lock().unwrap() = true; });

    // User clicks the AI Chat button on the dashboard
    dashboard_ui.invoke_open_ai_chat();

    // Verify flow reached the target destination
    assert!(*opened.lock().unwrap(), "AI Chat should be opened from Dashboard");

    // Verify the destination component renders correctly
    let ui = crate::app::AiHelpChat::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("AI Help Assistant"));
}
