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

    // In actual app flow, data is fetched via gRPC in an async task.
    // For this UI test, we manually inject the expected "System Activity" chart
    // that the backend would dynamically generate, ensuring the UI can render it.
    let generated_chart = vec![
        app::UiChartData {
            title: "System Activity".into(),
            points: slint::ModelRc::new(slint::VecModel::from(vec![
                app::UiDataPoint { label: "Total Msgs".into(), value: 15.0, display_value: "15".into() },
                app::UiDataPoint { label: "Audited".into(), value: 10.0, display_value: "10".into() },
                app::UiDataPoint { label: "Agents".into(), value: 3.0, display_value: "3".into() },
            ])),
        },
    ];
    analytics_ui.set_charts(slint::ModelRc::new(slint::VecModel::from(generated_chart)));

    // 3. User navigates from dashboard
    dashboard_ui.invoke_action_see_analytics();

    // Verify dashboard action
    assert!(*see_analytics_called.borrow(), "Analytics action should be invoked from Dashboard");

    // Verify the UI correctly received and holds the structured chart data
    let charts = analytics_ui.get_charts();
    assert_eq!(charts.row_count(), 1, "Should have 1 chart");
    let first_chart = charts.row_data(0).unwrap();
    assert_eq!(first_chart.title, "System Activity");

    let points = first_chart.points;
    assert_eq!(points.row_count(), 3, "Should have 3 points");
    let first_point = points.row_data(0).unwrap();
    assert_eq!(first_point.display_value, "15");
    assert_eq!(first_point.value, 15.0);
    assert_eq!(first_point.label, "Total Msgs");

    // 4. Test close functionality
    let close_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let close_called_clone = close_called.clone();

    analytics_ui.on_close(move || {
        *close_called_clone.borrow_mut() = true;
    });

    analytics_ui.invoke_close();
    assert!(*close_called.borrow(), "Close action should be triggered on analytics window");
}
