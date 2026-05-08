use slint::{ComponentHandle, SharedString};
use crate::app;

#[test]
fn test_scribe_tooltip_missing_id() {
    crate::ui_tests::init();
    let dashboard_ui = app::Dashboard::new().unwrap();

    // Set up the registry to handle requests properly mimicking the main app
    dashboard_ui.global::<app::TooltipRegistry>().on_request_tooltip_text(|id| {
        let text = crate::get_tooltip_text(id.as_str());
        text
    });

    let tr = dashboard_ui.global::<app::TooltipRegistry>();

    // Test a valid tooltip
    tr.invoke_show_tooltip("help_center".into(), 0.0, 0.0);
    assert!(tr.get_is_visible());
    assert_eq!(tr.get_active_text(), SharedString::from("Find answers and how-to guides."));

    // Test a missing ID, expecting it to clear or hide text, or text will be empty and is_visible remains true/false based on implementation.
    // Based on implementation: if (active_text != "") { ... is_visible = true }
    // which means if active_text == "", it shouldn't be set to true if it was false.
    tr.invoke_hide_tooltip();
    tr.invoke_show_tooltip("unknown_id".into(), 0.0, 0.0);
    assert_eq!(tr.get_active_text(), SharedString::from(""));
    assert!(!tr.get_is_visible());
}

#[test]
fn test_scribe_api_docs_test_endpoint_valid() {
    crate::ui_tests::init();
    let ui = app::ApiDocs::new().unwrap();

    assert!(!ui.get_is_advanced());
    ui.set_is_advanced(true);
    assert!(ui.get_is_advanced());

    let endpoint_tested = std::rc::Rc::new(std::cell::RefCell::new(false));
    let endpoint_tested_clone = endpoint_tested.clone();

    // Mock the callback
    ui.on_test_endpoint(move |_| {
        *endpoint_tested_clone.borrow_mut() = true;
    });

    ui.invoke_test_endpoint("/v2/users".into());
    assert!(*endpoint_tested.borrow());

    // In our test, since `invoke_test_endpoint` just invokes the callback, we also
    // manually test state mapping. Since `active_endpoint` isn't updated by `invoke_test_endpoint`
    // itself in Rust (it's bound to the click handler in Slint), we manually set it to verify state bindings.
    ui.set_active_endpoint("/v2/users".into());
    assert_eq!(ui.get_active_endpoint(), SharedString::from("/v2/users"));
}
