use slint::ComponentHandle;
use slint::Model;
use crate::app;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_geo_visibility_flow() {
    crate::ui_tests::init();
    let dashboard_ui = app::Dashboard::new().unwrap();

    let geo_opened = Rc::new(RefCell::new(false));
    let geo_opened_clone = geo_opened.clone();

    dashboard_ui.on_action_open_geo_visibility(move || {
        *geo_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_open_geo_visibility();
    assert!(*geo_opened.borrow(), "GEO Visibility tool should be opened from Dashboard");

    let geo_ui = app::GeoVisibility::new().unwrap();

    // Test 1: Initial state
    assert_eq!(geo_ui.get_generative_score(), "N/A");
    assert!(!geo_ui.get_is_scanning());
    assert_eq!(geo_ui.get_actionable_steps().row_count(), 0);

    let start_scan_called = Rc::new(RefCell::new(false));
    let start_scan_called_clone = start_scan_called.clone();

    let geo_handle = geo_ui.as_weak();
    geo_ui.on_start_scan(move || {
        *start_scan_called_clone.borrow_mut() = true;
        if let Some(ui) = geo_handle.upgrade() {
            ui.set_is_scanning(true);

            ui.set_is_scanning(false);
            ui.set_generative_score("65".into());
            let steps = vec![
                app::UiGeoRecommendation {
                    id: "geo-1".into(),
                    title: "Add Schema.org Markup".into(),
                    description: "Test description".into(),
                    impact: "High".into(),
                    is_applied: false,
                }
            ];
            ui.set_actionable_steps(slint::ModelRc::new(slint::VecModel::from(steps)));
        }
    });

    geo_ui.invoke_start_scan();
    // Test 2: Verify state after scan
    assert!(*start_scan_called.borrow(), "Scan should be triggered");
    assert_eq!(geo_ui.get_generative_score(), "65");
    assert_eq!(geo_ui.get_actionable_steps().row_count(), 1);

    let apply_called = Rc::new(RefCell::new(false));
    let apply_called_clone = apply_called.clone();

    let geo_handle_apply = geo_ui.as_weak();
    geo_ui.on_apply_recommendation(move |id| {
        *apply_called_clone.borrow_mut() = true;
        assert_eq!(id, "geo-1");
        if let Some(ui) = geo_handle_apply.upgrade() {
            let mut current_steps: Vec<app::UiGeoRecommendation> = ui.get_actionable_steps().iter().collect();
            for step in current_steps.iter_mut() {
                if step.id == id {
                    step.is_applied = true;
                }
            }
            ui.set_actionable_steps(slint::ModelRc::new(slint::VecModel::from(current_steps)));
            ui.set_generative_score("80".into());
        }
    });

    geo_ui.invoke_apply_recommendation("geo-1".into());
    // Test 3: Recommendation apply triggers and succeeds
    assert!(*apply_called.borrow(), "Recommendation apply should be triggered");

    // Test 4: Recommendation is marked as applied
    assert!(geo_ui.get_actionable_steps().row_data(0).unwrap().is_applied);

    // Test 5: Score increases
    assert_eq!(geo_ui.get_generative_score(), "80");
}
