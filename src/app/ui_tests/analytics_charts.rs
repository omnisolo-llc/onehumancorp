use crate::app;
use slint::{Model, ComponentHandle};

#[test]
fn test_analytics_charts_e2e_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. Setup UI Components
    let dashboard_ui = app::Dashboard::new().unwrap();
    let analytics_ui = app::AnalyticsCharts::new().unwrap();

    let see_analytics_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let see_analytics_called_clone = see_analytics_called.clone();

    // Wire up dashboard to analytics chart
    let analytics_handle = analytics_ui.as_weak();
    dashboard_ui.on_action_see_analytics(move || {
        *see_analytics_called_clone.borrow_mut() = true;
        if let Some(ui) = analytics_handle.upgrade() {
            let _ = ui.show();
        }
    });

    // Configure data conceptually for UI test by directly setting data similar to main.rs logic

    let charts = vec![
        app::UiChartData {
            title: "Analytics Overview".into(),
            points: slint::ModelRc::new(slint::VecModel::from(vec![
                app::UiDataPoint { label: "Total Agents".into(), value: 5.0, display_value: "5".into() },
                app::UiDataPoint { label: "Total Humans".into(), value: 10.0, display_value: "10".into() },
                app::UiDataPoint { label: "Fidelity %".into(), value: 95.5, display_value: "95.5%".into() },
            ])),
        },
        app::UiChartData {
            title: "Operational Stats".into(),
            points: slint::ModelRc::new(slint::VecModel::from(vec![
                app::UiDataPoint { label: "Latency (ms)".into(), value: 120.0, display_value: "120".into() },
                app::UiDataPoint { label: "Pending Approvals".into(), value: 3.0, display_value: "3".into() },
                app::UiDataPoint { label: "Active Handoffs".into(), value: 2.0, display_value: "2".into() },
                app::UiDataPoint { label: "Token Velocity".into(), value: 1500.0, display_value: "1500".into() },
            ])),
        },
    ];
    analytics_ui.set_charts(slint::ModelRc::new(slint::VecModel::from(charts)));

    // 3. User navigates from dashboard
    dashboard_ui.invoke_action_see_analytics();

    // Verify dashboard action
    assert!(*see_analytics_called.borrow(), "Analytics action should be invoked from Dashboard");

    // Verify chart data populated correctly
    let charts = analytics_ui.get_charts();
    assert_eq!(charts.row_count(), 2, "Should have 2 charts");
    let first_chart = charts.row_data(0).unwrap();
    assert_eq!(first_chart.title, "Analytics Overview");

    let points = first_chart.points;
    assert_eq!(points.row_count(), 3, "Should have 3 points in first chart");
    let first_point = points.row_data(0).unwrap();
    assert_eq!(first_point.display_value, "5");
    assert_eq!(first_point.value, 5.0);
    assert_eq!(first_point.label, "Total Agents");

    // 4. Test close functionality
    let close_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let close_called_clone = close_called.clone();

    analytics_ui.on_close(move || {
        *close_called_clone.borrow_mut() = true;
    });

    analytics_ui.invoke_close();
    assert!(*close_called.borrow(), "Close action should be triggered on analytics window");
}

#[test]
fn test_analytics_charts_empty_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let analytics_ui = app::AnalyticsCharts::new().unwrap();
    analytics_ui.set_charts(slint::ModelRc::new(slint::VecModel::from(vec![])));
    assert_eq!(analytics_ui.get_charts().row_count(), 0);
}

#[test]
fn test_analytics_charts_large_values() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let analytics_ui = app::AnalyticsCharts::new().unwrap();
    let charts = vec![
        app::UiChartData {
            title: "Large Stats".into(),
            points: slint::ModelRc::new(slint::VecModel::from(vec![
                app::UiDataPoint { label: "Huge".into(), value: 9999999.0, display_value: "10M".into() },
            ])),
        },
    ];
    analytics_ui.set_charts(slint::ModelRc::new(slint::VecModel::from(charts)));
    let set_charts = analytics_ui.get_charts();
    assert_eq!(set_charts.row_data(0).unwrap().points.row_data(0).unwrap().value, 9999999.0);
}

#[test]
fn test_analytics_charts_multiple_points() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let analytics_ui = app::AnalyticsCharts::new().unwrap();
    let charts = vec![
        app::UiChartData {
            title: "Timeline".into(),
            points: slint::ModelRc::new(slint::VecModel::from(vec![
                app::UiDataPoint { label: "1".into(), value: 1.0, display_value: "1".into() },
                app::UiDataPoint { label: "2".into(), value: 2.0, display_value: "2".into() },
                app::UiDataPoint { label: "3".into(), value: 3.0, display_value: "3".into() },
                app::UiDataPoint { label: "4".into(), value: 4.0, display_value: "4".into() },
            ])),
        },
    ];
    analytics_ui.set_charts(slint::ModelRc::new(slint::VecModel::from(charts)));
    let set_charts = analytics_ui.get_charts();
    assert_eq!(set_charts.row_data(0).unwrap().points.row_count(), 4);
}

#[test]
fn test_analytics_charts_negative_values() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let analytics_ui = app::AnalyticsCharts::new().unwrap();
    let charts = vec![
        app::UiChartData {
            title: "Negative".into(),
            points: slint::ModelRc::new(slint::VecModel::from(vec![
                app::UiDataPoint { label: "Loss".into(), value: -50.0, display_value: "-50".into() },
            ])),
        },
    ];
    analytics_ui.set_charts(slint::ModelRc::new(slint::VecModel::from(charts)));
    let set_charts = analytics_ui.get_charts();
    assert_eq!(set_charts.row_data(0).unwrap().points.row_data(0).unwrap().value, -50.0);
}
