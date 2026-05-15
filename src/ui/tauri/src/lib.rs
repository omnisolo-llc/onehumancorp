use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

macro_rules! tauri_build_context {
    () => {
        include!("../tauri-build-context.rs");
    };
}

tauri_build_context!();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("App").unwrap();
            Ok(())
        })
        .run(tauri_context())
        .expect("error while running tauri application");
}
