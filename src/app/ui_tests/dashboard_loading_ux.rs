use crate::app;

#[test]
fn test_dashboard_loading_state_toggles() {
    crate::ui_tests::init();
    let ui = app::Dashboard::new().unwrap();

    assert!(!ui.get_is_loading(), "Dashboard should not be loading by default");

    ui.set_is_loading(true);
    assert!(ui.get_is_loading(), "Dashboard should reflect loading state when toggled");

    ui.set_is_loading(false);
    assert!(!ui.get_is_loading(), "Dashboard should reflect non-loading state when toggled off");
}

#[test]
fn test_dashboard_loading_shimmer_properties() {
    crate::ui_tests::init();
    let ui = app::Dashboard::new().unwrap();

    ui.set_todays_sales("$1000".into());
    assert_eq!(ui.get_todays_sales(), "$1000", "Data should be preserved regardless of loading state");

    ui.set_is_loading(true);
    assert_eq!(ui.get_todays_sales(), "$1000", "Data should be preserved during loading state");
}

#[test]
fn test_dashboard_loading_persists_during_updates() {
    crate::ui_tests::init();
    let ui = app::Dashboard::new().unwrap();

    ui.set_is_loading(true);
    ui.set_new_orders_count(5);

    assert!(ui.get_is_loading(), "Loading state should persist when updating other properties");
    assert_eq!(ui.get_new_orders_count(), 5, "Updates should apply even while loading");
}

#[test]
fn test_dashboard_loading_and_milestone_independence() {
    crate::ui_tests::init();
    let ui = app::Dashboard::new().unwrap();

    ui.set_is_loading(true);
    ui.set_show_milestone(true);

    assert!(ui.get_is_loading(), "Loading state should not interfere with milestone state");
    assert!(ui.get_show_milestone(), "Milestone state should not interfere with loading state");
}

#[test]
fn test_dashboard_loading_advanced_mode_independence() {
    crate::ui_tests::init();
    let ui = app::Dashboard::new().unwrap();

    ui.set_is_loading(true);
    ui.set_is_advanced(true);

    assert!(ui.get_is_loading(), "Loading state should not be affected by advanced mode");
    assert!(ui.get_is_advanced(), "Advanced mode should be togglable during loading state");
}
