// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
#[tauri::command]
async fn close_splashscreen(app: tauri::AppHandle) {
    // Close the splashscreen
    if let Some(splash_window) = app.get_webview_window("login") {
        splash_window.close().unwrap();
    }
    // Show the main window
    if let Some(main_window) = app.get_webview_window("main") {
        main_window.show().unwrap();
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
