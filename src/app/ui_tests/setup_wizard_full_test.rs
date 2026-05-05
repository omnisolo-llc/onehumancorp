use crate::app;
#[test]
fn test_ui_setup_wizard_business_type_selection() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    super::init();
    let ui = app::SetupWizard::new().unwrap();
    assert_eq!(ui.get_step(), 0);
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);
    ui.set_business_type("Online Store".into());
    assert_eq!(ui.get_business_type(), "Online Store");
}
