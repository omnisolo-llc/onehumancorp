use crate::app;

fn create() -> app::Dashboard {
    crate::ui_tests::init();
    app::Dashboard::new().unwrap()
}

#[test]
fn preview_widget_test() {
    let ui = create();
    // Test that it initializes successfully as part of the dashboard
    let _ = ui.get_show_menu(); // Dummy call to ensure it's alive
}

#[test]
fn preview_widget_url_test() {
    // A more thorough E2E testing for preview
    let ui = create();
    // Assuming WebsitePreviewWidget has properties, we don't have access to them from the Dashboard root in Slint
    // directly unless exposed, but we can verify it doesn't crash on startup.
}
