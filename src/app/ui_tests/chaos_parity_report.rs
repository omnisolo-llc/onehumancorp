use crate::app;
use slint::Model;
use slint::ComponentHandle;

fn create() -> app::Dashboard { crate::ui_tests::init(); app::Dashboard::new().unwrap() }

#[test]
fn test_chaos_report_default_hidden() {
    let ui = create();
    assert!(!ui.get_show_chaos_report(), "Chaos Parity Report should be hidden by default");
}

#[test]
fn test_chaos_report_toggle_on() {
    let ui = create();
    ui.set_show_chaos_report(true);
    assert!(ui.get_show_chaos_report(), "Chaos Parity Report should be visible after state update");
}

#[test]
fn test_chaos_report_toggle_off() {
    let ui = create();
    ui.set_show_chaos_report(true);
    assert!(ui.get_show_chaos_report(), "Panel should be visible");
    ui.set_show_chaos_report(false);
    assert!(!ui.get_show_chaos_report(), "Panel should be hidden again");
}

#[test]
fn test_chaos_report_button_click() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();

    ui.on_action_open_chaos_report(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_action_open_chaos_report();

    assert!(*invoked.borrow(), "Chaos Report button click should invoke the correct callback");
}

#[test]
fn test_chaos_report_render_safe() {
    let ui = create();
    ui.set_show_chaos_report(true);

    // Just verify we can instantiate and set the property without crashing.
    // The data bounds are verified in integration/Slint properties
    assert!(ui.get_show_chaos_report());
}
