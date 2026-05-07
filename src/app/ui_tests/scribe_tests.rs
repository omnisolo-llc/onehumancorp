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
