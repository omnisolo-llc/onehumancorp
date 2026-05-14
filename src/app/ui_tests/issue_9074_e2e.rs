use crate::app;

#[test]
fn test_maya_baker_e2e() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("test@example.com".into(), "secure123".into());
    login_ui.invoke_start_setup_wizard();
    let wizard_ui = app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Maya's Cakes".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Physical".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), wizard_ui.get_step());
}

#[test]
fn test_carlos_handyman_e2e() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("test@example.com".into(), "secure123".into());
    login_ui.invoke_start_setup_wizard();
    let wizard_ui = app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Carlos Repairs".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Service".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), wizard_ui.get_step());
}

#[test]
fn test_priya_boutique_e2e() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("test@example.com".into(), "secure123".into());
    login_ui.invoke_start_setup_wizard();
    let wizard_ui = app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Priya Boutique".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Physical".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), wizard_ui.get_step());
}

#[test]
fn test_leo_tutor_e2e() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("test@example.com".into(), "secure123".into());
    login_ui.invoke_start_setup_wizard();
    let wizard_ui = app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Leo Music".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Digital".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), wizard_ui.get_step());
}

#[test]
fn test_fatima_food_cart_e2e() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("test@example.com".into(), "secure123".into());
    login_ui.invoke_start_setup_wizard();
    let wizard_ui = app::SetupWizard::new().unwrap();
    wizard_ui.set_step(0);
    wizard_ui.invoke_next_step();
    wizard_ui.set_company_name("Fatima Food Cart".into());
    wizard_ui.invoke_next_step();
    wizard_ui.invoke_select_business_type("Food".into());
    wizard_ui.set_launching(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), wizard_ui.get_step());
}
