#[test]
fn test_scribe_feature_dashboard_callbacks() {
    crate::ui_tests::init();
    let ui = crate::app::ScribeFeatureDashboard::new().unwrap();

    let open_help_center_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let open_help_center_called_clone = open_help_center_called.clone();
    ui.on_open_help_center(move || {
        *open_help_center_called_clone.borrow_mut() = true;
    });

    let open_ai_chat_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let open_ai_chat_called_clone = open_ai_chat_called.clone();
    ui.on_open_ai_chat(move || {
        *open_ai_chat_called_clone.borrow_mut() = true;
    });

    let open_walkthrough_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let open_walkthrough_called_clone = open_walkthrough_called.clone();
    ui.on_open_walkthrough(move || {
        *open_walkthrough_called_clone.borrow_mut() = true;
    });

    let open_video_tutorials_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let open_video_tutorials_called_clone = open_video_tutorials_called.clone();
    ui.on_open_video_tutorials(move || {
        *open_video_tutorials_called_clone.borrow_mut() = true;
    });

    let open_api_docs_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let open_api_docs_called_clone = open_api_docs_called.clone();
    ui.on_open_api_docs(move || {
        *open_api_docs_called_clone.borrow_mut() = true;
    });

    let open_release_notes_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let open_release_notes_called_clone = open_release_notes_called.clone();
    ui.on_open_release_notes(move || {
        *open_release_notes_called_clone.borrow_mut() = true;
    });

    ui.invoke_open_help_center();
    assert!(*open_help_center_called.borrow());

    ui.invoke_open_ai_chat();
    assert!(*open_ai_chat_called.borrow());

    ui.invoke_open_walkthrough();
    assert!(*open_walkthrough_called.borrow());

    ui.invoke_open_video_tutorials();
    assert!(*open_video_tutorials_called.borrow());

    ui.invoke_open_api_docs();
    assert!(*open_api_docs_called.borrow());

    ui.invoke_open_release_notes();
    assert!(*open_release_notes_called.borrow());
}
