#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn generate_password(options: password_core::Options) -> Result<String, String> {
    password_core::generate(&options)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![generate_password])
        .run(tauri::generate_context!())
        .expect("Die Anwendung konnte nicht gestartet werden");
}
