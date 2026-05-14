pub mod memory_commands;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

macro_rules! tauri_build_context {
    () => {

    };
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            crate::memory_commands::api_memory_get_metrics,
            crate::memory_commands::api_memory_get_records,
            crate::memory_commands::api_memory_trigger_sync,
            crate::memory_commands::api_memory_resolve_conflict,
            crate::memory_commands::api_memory_get_advanced_config,
            crate::memory_commands::api_memory_set_advanced_config,
            crate::memory_commands::api_memory_export_graph,
greet])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("App").unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}