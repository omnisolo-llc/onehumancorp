use crate::app;
use slint::{ComponentHandle, Model};

fn create() -> app::BusinessManager { crate::ui_tests::init(); app::BusinessManager::new().unwrap() }

#[test]
fn test_service_schedule_binding() {
    let ui = create();
    ui.set_service_schedule("Mon-Fri 9am-5pm".into());
    assert_eq!(ui.get_service_schedule(), "Mon-Fri 9am-5pm");
}

#[test]
fn test_step_transitions() {
    let ui = create();
    assert_eq!(ui.get_step(), 0);
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);
    ui.invoke_prev_step();
    assert_eq!(ui.get_step(), 0);
}

#[test]
fn test_type_selection() {
    let ui = create();
    ui.invoke_select_type("SERVICE".into());
    assert_eq!(ui.get_selected_type(), "SERVICE");
}

#[test]
fn test_service_duration_binding() {
    let ui = create();
    ui.set_service_duration("120".into());
    assert_eq!(ui.get_service_duration(), "120");
}

#[test]
fn test_submit_callback() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();
    ui.on_submit(move |type_, name, _desc, _price, _dur, sched| {
        assert_eq!(type_, "SERVICE");
        assert_eq!(name, "Consultation");
        *invoked_clone.borrow_mut() = true;
    });
    ui.invoke_submit("SERVICE".into(), "Consultation".into(), "".into(), "".into(), "".into(), "".into());
    assert!(*invoked.borrow());
}