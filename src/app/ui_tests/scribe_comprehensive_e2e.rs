use slint::{ComponentHandle, Model};

#[test]
fn test_comprehensive_scribe_features() {
    crate::ui_tests::init();

    // Help Center
    let help_ui = crate::app::HelpCenter::new().unwrap();
    let articles = help_ui.get_articles();
    assert!(articles.row_count() >= 7);
    help_ui.set_search_query("test".into());
    assert_eq!(help_ui.get_search_query(), slint::SharedString::from("test"));

    // AI Help Chat
    let ai_ui = crate::app::AiHelpChat::new().unwrap();
    let msgs = ai_ui.get_messages();
    assert!(msgs.row_count() >= 1);
    ai_ui.set_user_input("hello".into());
    assert_eq!(ai_ui.get_user_input(), slint::SharedString::from("hello"));

    // Interactive Walkthrough
    let walk_ui = crate::app::InteractiveWalkthrough::new().unwrap();
    walk_ui.set_current_step(1);
    assert_eq!(walk_ui.get_current_step(), 1);

    // Video Tutorials
    let video_ui = crate::app::VideoTutorials::new().unwrap();
    let videos = video_ui.get_videos();
    assert!(videos.row_count() >= 10);
    video_ui.set_is_playing(true);
    assert!(video_ui.get_is_playing());

    // API Docs
    let api_ui = crate::app::ApiDocs::new().unwrap();
    assert!(!api_ui.get_is_advanced());
    api_ui.set_is_advanced(true);
    assert!(api_ui.get_is_advanced());

    // Release Notes
    let notes_ui = crate::app::ReleaseNotes::new().unwrap();
    assert!(!notes_ui.get_show_latest_only());
    notes_ui.set_show_latest_only(true);
    assert!(notes_ui.get_show_latest_only());
}
