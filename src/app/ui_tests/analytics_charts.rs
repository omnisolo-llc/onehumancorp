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

    // 2. Data is now fetched dynamically, but for UI isolation tests we can directly inject
    let mock_charts = vec![
        app::UiChartData {
            title: "Revenue Over Time".into(),
            points: slint::ModelRc::new(slint::VecModel::from(vec![
                app::UiDataPoint { label: "Mon".into(), value: 40.0, display_value: "$400".into() },
            ])),
        },
    ];
    analytics_ui.set_charts(slint::ModelRc::new(slint::VecModel::from(mock_charts)));

    // 3. User navigates from dashboard
    dashboard_ui.invoke_action_see_analytics();

    // Verify dashboard action
    assert!(*see_analytics_called.borrow(), "Analytics action should be invoked from Dashboard");

    // Verify chart data populated correctly
    let charts = analytics_ui.get_charts();
    assert_eq!(charts.row_count(), 1, "Should have 1 chart");
    let first_chart = charts.row_data(0).unwrap();
    assert_eq!(first_chart.title, "Revenue Over Time");

    let points = first_chart.points;
    assert_eq!(points.row_count(), 1, "Should have 1 point");
    let first_point = points.row_data(0).unwrap();
    assert_eq!(first_point.display_value, "$400");
    assert_eq!(first_point.value, 40.0);
    assert_eq!(first_point.label, "Mon");

    // 4. Test close functionality
    let close_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let close_called_clone = close_called.clone();

    analytics_ui.on_close(move || {
        *close_called_clone.borrow_mut() = true;
    });

    analytics_ui.invoke_close();
    assert!(*close_called.borrow(), "Close action should be triggered on analytics window");
}
