use crate::app;

fn create() -> app::KairosOrchestrationWalkthrough { crate::ui_tests::init(); app::KairosOrchestrationWalkthrough::new().unwrap() }

#[test]
fn kairos_walkthrough_flow_steps_bounds() {
    let ui = create();
    ui.set_current_step(10);
    assert_eq!(ui.get_current_step(), 10);
    ui.set_current_step(-5);
    assert_eq!(ui.get_current_step(), -5);
}

#[test]
fn kairos_walkthrough_flow_visibility() {
    let ui = create();
    ui.set_current_step(3);
    assert_eq!(ui.get_current_step(), 3);
}

#[test]
fn create_verify_current_step() {
    let ui = create();
    ui.set_current_step(0);
    assert_eq!(ui.get_current_step(), 0);
    ui.set_current_step(1);
    assert_eq!(ui.get_current_step(), 1);
    ui.set_current_step(2);
    assert_eq!(ui.get_current_step(), 2);
}
