use crate::app;
use slint::ComponentHandle;

fn create() -> app::Login { crate::ui_tests::init(); app::Login::new().unwrap() }

// --- Security / Hacking Cases ---

#[test] fn login_sqli_username() {
    let ui = create();
    let malicious = "' OR '1'='1";
    ui.set_username(malicious.into());
    assert_eq!(ui.get_username(), malicious);
}

#[test] fn login_xss_password() {
    let ui = create();
    let xss = "<script>alert('pwned')</script>";
    ui.set_password(xss.into());
    assert_eq!(ui.get_password(), xss);
}

#[test] fn login_path_traversal() {
    let ui = create();
    ui.set_username("../../../etc/passwd".into());
    assert_eq!(ui.get_username(), "../../../etc/passwd");
}

#[test] fn login_null_byte() {
    let ui = create();
    ui.set_username("admin\0user".into());
    assert_eq!(ui.get_username(), "admin\0user");
}

#[test] fn login_unicode_homograph() {
    let ui = create();
    ui.set_username("аdmin".into()); // Cyrillic 'а'
    assert_eq!(ui.get_username(), "аdmin");
}

#[test] fn login_overflow_username() {
    let ui = create();
    let long = "A".repeat(10000);
    ui.set_username(long.clone().into());
    assert_eq!(ui.get_username(), long);
}

// --- Boundary / Corner Cases ---

#[test] fn login_empty_credentials() {
    let ui = create();
    ui.set_username("".into());
    ui.set_password("".into());
    assert_eq!(ui.get_username(), "");
    assert_eq!(ui.get_password(), "");
    ui.invoke_login("".into(), "".into());
}

#[test] fn login_max_int_username() {
    let ui = create();
    ui.set_username("2147483647".into());
    assert_eq!(ui.get_username(), "2147483647");
}

#[test] fn login_special_chars() {
    let ui = create();
    let chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    ui.set_username(chars.into());
    assert_eq!(ui.get_username(), chars);
}

// --- Complex Flows ---

#[test] fn login_flow_toggle_signup_multiple_times() {
    let ui = create();
    for _ in 0..10 {
        ui.set_is_sign_up(true);
        assert!(ui.get_is_sign_up());
        ui.set_is_sign_up(false);
        assert!(!ui.get_is_sign_up());
    }
}

#[test] fn login_flow_error_persistence() {
    let ui = create();
    ui.set_error_message("Initial error".into());
    ui.set_username("new_user".into());
    assert_eq!(ui.get_error_message(), "Initial error");
    ui.set_error_message("".into());
    assert_eq!(ui.get_error_message(), "");
}

#[test] fn login_callback_chain() {
    let ui = create();
    let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
    let c = counter.clone();
    ui.on_login(move |_, _| { *c.borrow_mut() += 1; });
    
    ui.invoke_login("u1".into(), "p1".into());
    ui.invoke_login("u2".into(), "p2".into());
    assert_eq!(*counter.borrow(), 2);
}

// --- Unique Data Tests with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_username() {
    let ui = create();
    ui.set_username("user@domain.com".into());
    assert_eq!(ui.get_username(), "user@domain.com");
    ui.set_username("user+tag@domain.com".into());
    assert_eq!(ui.get_username(), "user+tag@domain.com");
    ui.set_username("1234567890".into());
    assert_eq!(ui.get_username(), "1234567890");
}

#[test]
fn create_verify_password() {
    let ui = create();
    ui.set_password("                    ".into());
    assert_eq!(ui.get_password(), "                    ");
    ui.set_password("!@#$%^&*".into());
    assert_eq!(ui.get_password(), "!@#$%^&*");
    ui.set_password("LONG_STRING_WITH_NUMBERS_1234567890".into());
    assert_eq!(ui.get_password(), "LONG_STRING_WITH_NUMBERS_1234567890");
}

#[test]
fn create_verify_error_message() {
    let ui = create();
    ui.set_error_message("⚠️ Warning".into());
    assert_eq!(ui.get_error_message(), "⚠️ Warning");
    ui.set_error_message("Too many login attempts. Please try again later.".into());
    assert_eq!(ui.get_error_message(), "Too many login attempts. Please try again later.");
    ui.set_error_message("e61".into());
    assert_eq!(ui.get_error_message(), "e61");
}

#[test]
fn create_verify_verification_message() {
    let ui = create();
    ui.set_verification_message("123-456".into());
    assert_eq!(ui.get_verification_message(), "123-456");
    ui.set_verification_message("Please enter the 6-digit code sent to your phone.".into());
    assert_eq!(ui.get_verification_message(), "Please enter the 6-digit code sent to your phone.");
    ui.set_verification_message("v66".into());
    assert_eq!(ui.get_verification_message(), "v66");
}


#[test]
fn login_flow_first_login_shows_verification() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    let wizard = app::SetupWizard::new().unwrap();
    crate::configure_login_handlers(&ui, wizard.as_weak());

    ui.set_is_sign_up(true);
    ui.invoke_login("test@example.com".into(), "pass".into());

    assert_eq!(ui.get_show_verification(), true);
    assert_eq!(ui.get_verification_message(), "Please check your email to verify your account.");
}

#[test]
fn login_flow_first_login_verified_redirect() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    let wizard = app::SetupWizard::new().unwrap();
    crate::configure_login_handlers(&ui, wizard.as_weak());

    ui.invoke_resend_verification("test@example.com".into());

    assert!(!ui.window().is_visible());
    // wizard visibility cannot be asserted easily without event loop iteration
}

#[test]
fn login_flow_oauth_signup_redirect() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    let wizard = app::SetupWizard::new().unwrap();
    crate::configure_login_handlers(&ui, wizard.as_weak());

    ui.set_is_sign_up(true);
    ui.invoke_oauth_login("Google".into());

    assert!(!ui.window().is_visible());
    assert_eq!(ui.get_show_verification(), false);
}

#[tokio::test]
async fn login_flow_normal_login_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    let wizard = app::SetupWizard::new().unwrap();
    crate::configure_login_handlers(&ui, wizard.as_weak());

    ui.set_is_sign_up(false);

    let timer = slint::Timer::default();
    let ui_weak = ui.as_weak();
    let wizard_weak = wizard.as_weak();
    timer.start(slint::TimerMode::SingleShot, std::time::Duration::from_millis(500), move || {
        if let Some(u) = ui_weak.upgrade() {
            assert_eq!(u.get_show_verification(), false);
        }
        if let Some(w) = wizard_weak.upgrade() {
            // It should default to showing wizard since backend fails in test mock
            assert!(w.window().is_visible());
        }
        let _ = slint::quit_event_loop();
    });

    ui.invoke_login("user".into(), "pass".into());
    let _ = ui.run();
}

#[tokio::test]
async fn login_flow_normal_oauth_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::Login::new().unwrap();
    let wizard = app::SetupWizard::new().unwrap();
    crate::configure_login_handlers(&ui, wizard.as_weak());

    ui.set_is_sign_up(false);

    let timer = slint::Timer::default();
    let wizard_weak = wizard.as_weak();
    timer.start(slint::TimerMode::SingleShot, std::time::Duration::from_millis(500), move || {
        if let Some(w) = wizard_weak.upgrade() {
            assert!(w.window().is_visible());
        }
        let _ = slint::quit_event_loop();
    });

    ui.invoke_oauth_login("Apple".into());
    let _ = ui.run();
}
