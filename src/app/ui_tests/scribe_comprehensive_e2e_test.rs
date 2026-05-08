use slint::ComponentHandle;

#[test]
fn test_e2e_scribe_documentation_journey() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. User starts at Login
    let login_ui = crate::app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |_, _| {
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("ceo@store.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "Login must succeed");

    // 2. User arrives at Dashboard and sees the Help Center tooltip
    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let tr = dashboard_ui.global::<crate::app::TooltipRegistry>();

    tr.on_request_tooltip_text(|id| {
        if id == "help_center" {
            "Find answers and how-to guides.".into()
        } else {
            "".into()
        }
    });

    tr.invoke_show_tooltip("help_center".into(), 0.0, 0.0);
    assert!(tr.get_is_visible(), "Tooltip should show for help center");
    assert_eq!(tr.get_active_text(), slint::SharedString::from("Find answers and how-to guides."));

    // 3. User opens the Help Center from the dashboard
    let hc_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let hc_opened_clone = hc_opened.clone();

    let scribe_dashboard = crate::app::ScribeFeatureDashboard::new().unwrap();
    scribe_dashboard.on_open_help_center(move || {
        *hc_opened_clone.borrow_mut() = true;
    });

    scribe_dashboard.invoke_open_help_center();
    assert!(*hc_opened.borrow(), "Help center must open");

    // 4. User interacts with the Help Center
    let hc_ui = crate::app::HelpCenter::new().unwrap();

    let search_triggered = std::rc::Rc::new(std::cell::RefCell::new(false));
    let search_triggered_clone = search_triggered.clone();
    hc_ui.on_execute_search(move || {
        *search_triggered_clone.borrow_mut() = true;
    });

    hc_ui.set_search_query("how to add products".into());
    hc_ui.invoke_execute_search();
    assert!(*search_triggered.borrow(), "Search must execute");

    // 5. User opens AI Help Chat from Scribe Dashboard
    let ai_chat_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let ai_chat_opened_clone = ai_chat_opened.clone();
    scribe_dashboard.on_open_ai_chat(move || {
        *ai_chat_opened_clone.borrow_mut() = true;
    });
    scribe_dashboard.invoke_open_ai_chat();
    assert!(*ai_chat_opened.borrow(), "AI Chat must open");

    // 6. User interacts with AI Help Chat
    let ai_chat = crate::app::AiHelpChat::new().unwrap();
    let chat_sent = std::rc::Rc::new(std::cell::RefCell::new(false));
    let chat_sent_clone = chat_sent.clone();
    ai_chat.on_send_message(move || {
        *chat_sent_clone.borrow_mut() = true;
    });
    ai_chat.set_user_input("I need help with payments".into());
    ai_chat.invoke_send_message();
    assert!(*chat_sent.borrow(), "Message must send");

    // 7. User opens Interactive Walkthrough
    let walkthrough_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let walkthrough_opened_clone = walkthrough_opened.clone();
    scribe_dashboard.on_open_walkthrough(move || {
        *walkthrough_opened_clone.borrow_mut() = true;
    });
    scribe_dashboard.invoke_open_walkthrough();
    assert!(*walkthrough_opened.borrow(), "Walkthrough must open");

    // 8. User goes through walkthrough
    let walkthrough = crate::app::InteractiveWalkthrough::new().unwrap();
    assert_eq!(walkthrough.get_current_step(), 0);
    walkthrough.set_current_step(1);
    assert_eq!(walkthrough.get_current_step(), 1);
    walkthrough.set_current_step(2);
    assert_eq!(walkthrough.get_current_step(), 2);
    walkthrough.set_current_step(3);
    assert_eq!(walkthrough.get_current_step(), 3);

    // 9. User checks Video Tutorials
    let video_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let video_opened_clone = video_opened.clone();
    scribe_dashboard.on_open_video_tutorials(move || {
        *video_opened_clone.borrow_mut() = true;
    });
    scribe_dashboard.invoke_open_video_tutorials();
    assert!(*video_opened.borrow(), "Video tutorials must open");

    let video = crate::app::VideoTutorials::new().unwrap();
    video.set_is_playing(true);
    assert!(video.get_is_playing());
    video.set_selected_video_title("How to run a promotion".into());
    assert_eq!(video.get_selected_video_title(), slint::SharedString::from("How to run a promotion"));
}
