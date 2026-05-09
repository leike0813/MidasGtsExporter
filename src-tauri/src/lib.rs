pub mod commands;
pub mod error;
pub mod exporters;
pub mod fpn_reader;
pub mod model;

#[cfg(feature = "app")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![commands::convert_mesh])
        .run(tauri::generate_context!())
        .expect("failed to run Tauri application");
}
