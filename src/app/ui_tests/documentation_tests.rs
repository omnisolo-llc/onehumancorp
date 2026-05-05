use crate::app::{HelpCenter, AiHelpChat, VideoTutorials};
use slint::{ComponentHandle, Model};

#[test]
fn test_help_center_instantiation() {
    crate::ui_tests::init();
    let help_center = HelpCenter::new().unwrap();
    assert!(help_center.get_articles().row_count() > 0, "Help Center must initialize with default articles");
}

#[test]
fn test_ai_help_chat_instantiation() {
    crate::ui_tests::init();
    let _ai_chat = AiHelpChat::new().unwrap();
    assert!(true);
}

#[test]
fn test_video_tutorials_instantiation() {
    crate::ui_tests::init();
    let _video_tutorials = VideoTutorials::new().unwrap();
    assert!(true);
}
