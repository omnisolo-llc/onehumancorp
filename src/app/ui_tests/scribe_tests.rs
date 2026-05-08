use slint::Model;
use crate::app;

#[test]
fn test_scribe_dashboard_quick_actions_hint_tooltip() {
    crate::ui_tests::init();
    let dashboard_ui = app::Dashboard::new().unwrap();

    assert_eq!(dashboard_ui.get_show_quick_actions_hint(), false);
    dashboard_ui.set_show_quick_actions_hint(true);
    assert_eq!(dashboard_ui.get_show_quick_actions_hint(), true);
}

#[test]
fn test_scribe_video_tutorials_url() {
    crate::ui_tests::init();
    let ui = app::VideoTutorials::new().unwrap();
    let videos = ui.get_videos();

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
fn test_scribe_ai_help_chat_style() {
    crate::ui_tests::init();
    let ui = app::AiHelpChat::new().unwrap();
    assert!(ui.get_messages().row_count() > 0);
}
