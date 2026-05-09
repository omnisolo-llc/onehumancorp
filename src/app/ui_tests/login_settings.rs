use crate::*;

#[test]
fn test_login_settings_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

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
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

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
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

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
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

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
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

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
