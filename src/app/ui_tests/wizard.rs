use crate::app;

fn create() -> app::SetupWizard {
    crate::ui_tests::init();
    let ui = app::SetupWizard::new().unwrap();
    ui.on_save_state(|| {});
    ui
}

#[test]
fn test_wizard_state_progression() {
    crate::ui_tests::init();
    let wizard = app::Wizard::new().unwrap();

    // Initial state
    assert_eq!(wizard.get_step(), 0);

    // Move to step 1
    wizard.invoke_next_step();
    assert_eq!(wizard.get_step(), 1);

    // Move to step 2
    wizard.invoke_next_step();
    assert_eq!(wizard.get_step(), 2);

    // Move back to step 1
    wizard.invoke_prev_step();
    assert_eq!(wizard.get_step(), 1);
}

#[test]
fn wizard_xss_company_name() {
    let ui = create();
    ui.set_company_name("<script>alert('xss')</script>".into());
    assert_eq!(ui.get_company_name(), "<script>alert('xss')</script>");
}

#[test]
fn wizard_unicode_launch_status() {
    let ui = create();
    ui.set_launch_status("🚀 測試 !".into());
    assert_eq!(ui.get_launch_status(), "🚀 測試 !");
}
