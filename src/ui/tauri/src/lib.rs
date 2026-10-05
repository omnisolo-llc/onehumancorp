#[cfg(desktop)]
mod native_runtime;
pub mod offline;

use tauri::Manager;

// The maintained web UI uses authenticated backend routes. Do not reintroduce
// the old static-export IPC handlers: they wrote plaintext provider keys and
// generated unauthenticated business-state placeholders. Existing user files
// are deliberately left untouched; reconnect through verified Integrations.

#[cfg(any(mobile, test))]
fn mobile_workspace_url(value: &str) -> Result<tauri::Url, String> {
    let url = tauri::Url::parse(value).map_err(|error| error.to_string())?;
    if url.scheme() != "https" || !url.username().is_empty() || url.password().is_some() {
        return Err("Mobile workspace URL must be HTTPS without credentials".into());
    }
    Ok(url)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .ok_or("Main window is unavailable")?;
            window.set_title("OmniSolo")?;
            #[cfg(desktop)]
            {
                app.handle()
                    .plugin(tauri_plugin_updater::Builder::new().build())?;
                app.manage(native_runtime::NativeRuntime::default());
                native_runtime::start_window(app.handle().clone());
            }
            #[cfg(mobile)]
            {
                let value = option_env!("OMNISOLO_MOBILE_WEB_URL")
                    .ok_or("Mobile workspace URL was not configured at build time")?;
                window.navigate(mobile_workspace_url(value)?)?;
            }
            Ok(())
        })
        .build(context)
        .expect("error while building tauri application")
        .run(|app, event| {
            #[cfg(desktop)]
            if matches!(event, tauri::RunEvent::Exit) {
                app.state::<native_runtime::NativeRuntime>().stop();
            }
            #[cfg(mobile)]
            let _ = (app, event);
        });
}

#[cfg(test)]
mod tests {
    use super::mobile_workspace_url;

    #[test]
    fn mobile_workspace_preserves_configured_https_location() {
        for value in [
            "https://workspace.example.com/",
            "https://workspace.example.com/app",
        ] {
            assert_eq!(mobile_workspace_url(value).unwrap().as_str(), value);
        }
    }

    #[test]
    fn mobile_workspace_rejects_http_credentials_and_malformed_urls() {
        for value in [
            "http://workspace.example.com",
            "http://127.0.0.1",
            "https://user@workspace.example.com",
            "https://user:password@workspace.example.com",
            "https://:password@workspace.example.com",
            "file:///tmp/app",
            "workspace.example.com",
            "https://[invalid]",
            "",
        ] {
            assert!(mobile_workspace_url(value).is_err(), "{value}");
        }
    }
}
