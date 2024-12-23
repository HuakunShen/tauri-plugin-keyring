// Learn more about Tauri commands at https://v2.tauri.app/develop/calling-rust/#commands

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .plugin(tauri_plugin_keyring::init())
        .setup(|app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
