use crate::app;
use slint::Model;

fn create() -> app::Analytics { crate::ui_tests::init(); app::Analytics::new().unwrap() }

#[test]
fn test_analytics_default_state() {
    let ui = create();
    assert_eq!(ui.get_generative_score(), 0);
    assert_eq!(ui.get_analysis_summary(), "Loading analysis...");
    assert_eq!(ui.get_actionable_steps().row_count(), 0);
}

#[test]
fn test_analytics_set_score() {
    let ui = create();
    ui.set_generative_score(85);
    assert_eq!(ui.get_generative_score(), 85);
}

#[test]
fn test_analytics_set_analysis_summary() {
    let ui = create();
    let summary = "Strong basic schema but lacks specific service offerings.";
    ui.set_analysis_summary(summary.into());
    assert_eq!(ui.get_analysis_summary(), summary);
}

#[test]
fn test_analytics_set_actionable_steps() {
    let ui = create();
    let steps = vec![
        "Add a plain-language FAQ section".into(),
        "Ensure pricing is explicitly labeled".into(),
    ];
    let model = std::rc::Rc::new(slint::VecModel::from(steps));
    ui.set_actionable_steps(model.into());
    assert_eq!(ui.get_actionable_steps().row_count(), 2);
}

#[test]
fn test_analytics_close_callback() {
    let ui = create();
    let closed = std::rc::Rc::new(std::cell::RefCell::new(false));
    let closed_clone = closed.clone();

    ui.on_close(move || {
        *closed_clone.borrow_mut() = true;
    });

    ui.invoke_close();
    assert!(*closed.borrow());
}
