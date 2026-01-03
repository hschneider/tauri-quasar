// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use tauri::Manager;
use tauri::WebviewUrl;
use tauri::AppHandle;

#[tauri::command]
fn ping() -> String {
    format!("PONG from the Rust-Backend!")
}

#[tauri::command]
fn clone_window(
      app: AppHandle,
      width: u32,
      height: u32,
      x: i32,
      y: i32,
      url: String,
) {
      let label = format!("clone-{}", uuid::Uuid::new_v4());
      let t = app.package_info().name.as_str();
      let title = format!("{} (Clone)", t );

      tauri::WebviewWindowBuilder::new(
        &app,
        label,
        WebviewUrl::App(url.into()),
      )
      .inner_size(width as f64, height as f64)
      .position(x as f64, y as f64) // offset
      .title(title)
      .build()
      .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping, clone_window])
        .setup(|app| {

            // Delete app cache on startup
            // to fix whitescreen issue
            //
            if cfg!(target_os = "linux") {
                let cache_dir = app.path().cache_dir()?;
                let package_info = app.package_info();
                let app_name = package_info.name.as_str();
                let app_cache = cache_dir.join(app_name);
                if app_cache.exists() {
                    let _ = fs::remove_dir_all(&app_cache);
                }
            }

            Ok(())
        })

        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

