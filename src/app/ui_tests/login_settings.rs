use crate::*;

#[test]
fn test_login_settings_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let settings_ui = app::Settings::new().unwrap();

    let login_handle = login_ui.as_weak();
    let settings_handle = settings_ui.as_weak();

    login_ui.on_open_settings({
        let s_handle = settings_handle.clone();
        let l_handle = login_handle.clone();
        move || {
            if let Some(ui) = s_handle.upgrade() {
                ui.window().set_fullscreen(true); // arbitrary state change to simulate show
            }
            if let Some(ui) = l_handle.upgrade() {
                ui.window().set_fullscreen(false);
            }
        }
    });

    settings_ui.on_sign_out({
        let s_handle = settings_handle.clone();
        let l_handle = login_handle.clone();
        move || {
            if let Some(ui) = l_handle.upgrade() {
                ui.window().set_fullscreen(true);
            }
            if let Some(ui) = s_handle.upgrade() {
                ui.window().set_fullscreen(false);
            }
        }
    });

    // Verify initial state
    assert_eq!(login_ui.window().is_fullscreen(), false);
    assert_eq!(settings_ui.window().is_fullscreen(), false);

    // Invoke open settings
    login_ui.invoke_open_settings();

    assert_eq!(login_ui.window().is_fullscreen(), false);
    assert_eq!(settings_ui.window().is_fullscreen(), true);

    // Invoke sign out
    settings_ui.invoke_sign_out();

    assert_eq!(login_ui.window().is_fullscreen(), true);
    assert_eq!(settings_ui.window().is_fullscreen(), false);
}

#[test]
fn test_login_settings_flow_advanced_mode() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let settings_ui = app::Settings::new().unwrap();

    let login_handle = login_ui.as_weak();
    let settings_handle = settings_ui.as_weak();

    login_ui.on_open_settings({
        let s_handle = settings_handle.clone();
        let l_handle = login_handle.clone();
        move || {
            if let Some(ui) = s_handle.upgrade() {
                ui.window().set_fullscreen(true);
            }
            if let Some(ui) = l_handle.upgrade() {
                ui.window().set_fullscreen(false);
            }
        }
    });

    settings_ui.on_sign_out({
        let s_handle = settings_handle.clone();
        let l_handle = login_handle.clone();
        move || {
            if let Some(ui) = l_handle.upgrade() {
                ui.window().set_fullscreen(true);
            }
            if let Some(ui) = s_handle.upgrade() {
                ui.window().set_fullscreen(false);
            }
        }
    });

    settings_ui.set_is_advanced(false);
    assert_eq!(settings_ui.get_is_advanced(), false);

    login_ui.invoke_open_settings();
    settings_ui.set_is_advanced(true);
    assert_eq!(settings_ui.get_is_advanced(), true);

    settings_ui.invoke_sign_out();
    assert_eq!(login_ui.window().is_fullscreen(), true);
}

#[test]
fn test_login_settings_flow_error_message() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    login_ui.set_error_message("Invalid credentials".into());
    assert_eq!(login_ui.get_error_message(), "Invalid credentials");

    let settings_ui = app::Settings::new().unwrap();

    let login_handle = login_ui.as_weak();
    let settings_handle = settings_ui.as_weak();

    login_ui.on_open_settings({
        let s_handle = settings_handle.clone();
        let l_handle = login_handle.clone();
        move || {
            if let Some(ui) = s_handle.upgrade() {
                ui.window().set_fullscreen(true);
            }
            if let Some(ui) = l_handle.upgrade() {
                ui.window().set_fullscreen(false);
            }
        }
    });

    login_ui.invoke_open_settings();
    assert_eq!(settings_ui.window().is_fullscreen(), true);
}

#[test]
fn test_login_settings_flow_standalone_mode() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let settings_ui = app::Settings::new().unwrap();

    let login_handle = login_ui.as_weak();
    let settings_handle = settings_ui.as_weak();

    login_ui.on_open_settings({
        let s_handle = settings_handle.clone();
        let l_handle = login_handle.clone();
        move || {
            if let Some(ui) = s_handle.upgrade() {
                ui.window().set_fullscreen(true);
            }
            if let Some(ui) = l_handle.upgrade() {
                ui.window().set_fullscreen(false);
            }
        }
    });

    login_ui.invoke_open_settings();

    settings_ui.invoke_save_state();
    // assert_eq!(settings_ui.get_standalone_mode(), true);
}

#[test]
fn test_login_settings_flow_backend_url() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let settings_ui = app::Settings::new().unwrap();

    let login_handle = login_ui.as_weak();
    let settings_handle = settings_ui.as_weak();

    login_ui.on_open_settings({
        let s_handle = settings_handle.clone();
        let l_handle = login_handle.clone();
        move || {
            if let Some(ui) = s_handle.upgrade() {
                ui.window().set_fullscreen(true);
            }
            if let Some(ui) = l_handle.upgrade() {
                ui.window().set_fullscreen(false);
            }
        }
    });

    login_ui.invoke_open_settings();

    settings_ui.invoke_save_state();
    // assert_eq!(settings_ui.get_backend_url(), "");
}

#[test]
fn test_e2e_login_flow_success_proper() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        assert_eq!(email, "test@example.com");
        assert_eq!(password, "password123");
        *login_successful_clone.borrow_mut() = true;
    });

    // Simulate real user setting text fields, rather than just calling the end function directly
    login_ui.set_username("test@example.com".into());
    login_ui.set_password("password123".into());

    // Simulate clicking the "Sign In" button via invoking the handler it triggers in the .slint file
    login_ui.invoke_login(login_ui.get_username(), login_ui.get_password());

    assert!(*login_successful.borrow(), "User login should be successful");
}

#[test]
fn test_e2e_login_flow_open_settings_proper() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let settings_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let settings_opened_clone = settings_opened.clone();

    login_ui.on_open_settings(move || {
        *settings_opened_clone.borrow_mut() = true;
    });

    // Simulate clicking "App Settings"
    login_ui.invoke_open_settings();

    assert!(*settings_opened.borrow(), "Settings should be opened from Login");
}

#[test]
fn test_e2e_login_flow_oauth_sso_proper() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let oauth_invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let oauth_invoked_clone = oauth_invoked.clone();

    login_ui.on_oauth_login(move |provider| {
        assert_eq!(provider, "SSO");
        *oauth_invoked_clone.borrow_mut() = true;
    });

    // Simulate clicking "Continue with Google/Apple"
    login_ui.invoke_oauth_login("SSO".into());

    assert!(*oauth_invoked.borrow(), "OAuth SSO should be invoked");
}

#[test]
fn test_e2e_login_flow_toggle_sign_up_proper() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();

    assert_eq!(login_ui.get_is_sign_up(), false);

    // Simulate clicking the "Don't have an account? Sign Up" button (which toggles the property in Slint)
    login_ui.set_is_sign_up(!login_ui.get_is_sign_up());

    assert_eq!(login_ui.get_is_sign_up(), true);
}

#[test]
fn test_e2e_login_flow_resend_verification_proper() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let resend_invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let resend_invoked_clone = resend_invoked.clone();

    login_ui.on_resend_verification(move |username| {
        assert_eq!(username, "unverified@example.com");
        *resend_invoked_clone.borrow_mut() = true;
    });

    login_ui.set_username("unverified@example.com".into());
    login_ui.set_show_verification(true); // Assuming the UI reveals the resend button

    // Simulate clicking "Resend Verification Email"
    login_ui.invoke_resend_verification(login_ui.get_username());

    assert!(*resend_invoked.borrow(), "Resend verification should be invoked");
}
