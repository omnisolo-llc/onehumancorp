use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_advanced_mode_wizard() {
    crate::ui_tests::init();
    let ui = crate::app::Wizard::new().unwrap();

    assert_eq!(ui.get_is_advanced(), false);
    ui.invoke_toggle_advanced();
    assert_eq!(ui.get_is_advanced(), true);
}
