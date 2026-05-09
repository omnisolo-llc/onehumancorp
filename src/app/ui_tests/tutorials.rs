use crate::app;

fn create() -> app::VideoTutorials {
    crate::ui_tests::init();
    app::VideoTutorials::new().unwrap()
}

// --- Specialized / Flow Tests ---

#[test]
fn tutorials_flow_playback() {
    let ui = create();
    ui.set_selected_video_title("How to Scale".into());
    ui.set_is_playing(true);
    assert_eq!(ui.get_selected_video_title(), "How to Scale");
    assert!(ui.get_is_playing());
    ui.set_is_playing(false);
    assert!(!ui.get_is_playing());
}

#[test]
fn tutorials_xss_title() {
    let ui = create();
    let xss = "<iframe src=javascript:alert('tutorial')>";
    ui.set_selected_video_title(xss.into());
    assert_eq!(ui.get_selected_video_title(), xss);
}

#[test]
fn tutorials_injection_title() {
    let ui = create();
    let inj = "Intro'); DROP TABLE tutorials; --";
    ui.set_selected_video_title(inj.into());
    assert_eq!(ui.get_selected_video_title(), inj);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_selected_video_title() {
    let ui = create();
    ui.set_selected_video_title("Basic Setup".into());
    assert_eq!(ui.get_selected_video_title(), "Basic Setup");
    ui.set_selected_video_title("Advanced Agents".into());
    assert_eq!(ui.get_selected_video_title(), "Advanced Agents");
    ui.set_selected_video_title("Billing Help".into());
    assert_eq!(ui.get_selected_video_title(), "Billing Help");
}

#[test]
fn create_verify_is_playing() {
    let ui = create();
    ui.set_is_playing(true);
    assert_eq!(ui.get_is_playing(), true);
    ui.set_is_playing(false);
    assert_eq!(ui.get_is_playing(), false);
}
