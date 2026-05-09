use crate::app;

fn create() -> app::Settings {
    crate::ui_tests::init();
    app::Settings::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn settings_email_injection() {
    let ui = create();
    let inj = "user@test.com'; DROP TABLE users; --";
    ui.set_user_email(inj.into());
    assert_eq!(ui.get_user_email(), inj);
}

#[test]
fn settings_org_id_overflow() {
    let ui = create();
    let long = "ORG-".to_string() + &"1".repeat(1000);
    ui.set_org_id(long.clone().into());
    assert_eq!(ui.get_org_id(), long);
}

#[test]
fn settings_xss_username() {
    let ui = create();
    let xss = "<body onload=alert(document.cookie)>";
    ui.set_user_name(xss.into());
    assert_eq!(ui.get_user_name(), xss);
}

#[test]
fn settings_invalid_role() {
    let ui = create();
    ui.set_user_role("INVALID_ROLE_STATE".into());
    assert_eq!(ui.get_user_role(), "INVALID_ROLE_STATE");
}

// --- Interaction / Flow Tests ---

#[test]
fn settings_full_profile_update_flow() {
    let ui = create();
    ui.set_user_name("Alice".into());
    ui.set_user_email("alice@example.com".into());
    ui.set_org_id("O-123".into());
    ui.set_user_role("Developer".into());
    assert_eq!(ui.get_user_name(), "Alice");
    assert_eq!(ui.get_user_email(), "alice@example.com");
    assert_eq!(ui.get_org_id(), "O-123");
    assert_eq!(ui.get_user_role(), "Developer");
}

#[test]
fn settings_service_status_toggle_flow() {
    let ui = create();
    for _ in 0..20 {
        ui.set_local_service_running(true);
        assert!(ui.get_local_service_running());
        ui.set_local_service_running(false);
        assert!(!ui.get_local_service_running());
    }
}

// --- Unique Scenarios ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_user_name() {
    let ui = create();
    ui.set_user_name("John Doe".into());
    assert_eq!(ui.get_user_name(), "John Doe");
    ui.set_user_name("".into());
    assert_eq!(ui.get_user_name(), "");
    ui.set_user_name("u11".into());
    assert_eq!(ui.get_user_name(), "u11");
}

#[test]
fn create_verify_user_email() {
    let ui = create();
    ui.set_user_email("john@doe.com".into());
    assert_eq!(ui.get_user_email(), "john@doe.com");
    ui.set_user_email("invalid-email".into());
    assert_eq!(ui.get_user_email(), "invalid-email");
    ui.set_user_email("e21@t.c".into());
    assert_eq!(ui.get_user_email(), "e21@t.c");
}

#[test]
fn create_verify_org_id() {
    let ui = create();
    ui.set_org_id("ORG-001".into());
    assert_eq!(ui.get_org_id(), "ORG-001");
    ui.set_org_id("12345".into());
    assert_eq!(ui.get_org_id(), "12345");
    ui.set_org_id("o31".into());
    assert_eq!(ui.get_org_id(), "o31");
}

#[test]
fn create_verify_user_role() {
    let ui = create();
    ui.set_user_role("Admin".into());
    assert_eq!(ui.get_user_role(), "Admin");
    ui.set_user_role("Guest".into());
    assert_eq!(ui.get_user_role(), "Guest");
    ui.set_user_role("r36".into());
    assert_eq!(ui.get_user_role(), "r36");
}
