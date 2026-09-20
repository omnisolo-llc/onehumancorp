#[cfg(desktop)]
mod native_runtime;
pub mod offline;

use tauri::Manager;

// The maintained web UI uses authenticated backend routes. Do not reintroduce
// the old static-export IPC handlers: they wrote plaintext provider keys and
// generated unauthenticated business-state placeholders. Existing user files
// are deliberately left untouched; reconnect through verified Integrations.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .ok_or("Main window is unavailable")?;
            window.set_title("OmniSolo")?;
            #[cfg(desktop)]
            {
                app.manage(native_runtime::NativeRuntime::default());
                native_runtime::start_window(app.handle().clone());
            }
            #[cfg(mobile)]
            {
                let value = option_env!("OMNISOLO_MOBILE_WEB_URL")
                    .ok_or("Mobile workspace URL was not configured at build time")?;
                let url = reqwest::Url::parse(value)?;
                if url.scheme() != "https" || !url.username().is_empty() || url.password().is_some()
                {
                    return Err("Mobile workspace URL must be HTTPS without credentials".into());
                }
                window.navigate(url)?;
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
