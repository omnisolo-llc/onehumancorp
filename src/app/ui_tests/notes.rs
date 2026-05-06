use crate::app;

fn create() -> app::ReleaseNotes { crate::ui_tests::init(); app::ReleaseNotes::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn notes_flow_version_sync() {
    let ui = create();
    ui.set_current_version("v1.0.0".into());
    assert_eq!(ui.get_current_version(), "v1.0.0");
}

#[test] fn notes_flow_toggle_latest() {
    let ui = create();
    ui.set_show_latest_only(true);
    assert!(ui.get_show_latest_only());
    ui.set_show_latest_only(false);
    assert!(!ui.get_show_latest_only());
}

#[test] fn notes_xss_version() {
    let ui = create();
    let xss = "<script>alert('version')</script>";
    ui.set_current_version(xss.into());
    assert_eq!(ui.get_current_version(), xss);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_current_version() {
    let ui = create();
    ui.set_current_version("v2.4.1".into());
    assert_eq!(ui.get_current_version(), "v2.4.1");
    ui.set_current_version("BETA-7".into());
    assert_eq!(ui.get_current_version(), "BETA-7");
    ui.set_current_version("ALPHA-RC1".into());
    assert_eq!(ui.get_current_version(), "ALPHA-RC1");
}

#[test]
fn create_verify_show_latest_only() {
    let ui = create();
    ui.set_show_latest_only(true);
    assert_eq!(ui.get_show_latest_only(), true);
    ui.set_show_latest_only(false);
    assert_eq!(ui.get_show_latest_only(), false);
}

#[test] fn notes_extra_validation() {
    let ui = create();
    ui.set_current_version("Test".into());
    assert_eq!(ui.get_current_version(), "Test");
}

#[test] fn notes_extra_validation_two() {
    let ui = create();
    ui.set_current_version("Another test".into());
    assert_eq!(ui.get_current_version(), "Another test");
}
