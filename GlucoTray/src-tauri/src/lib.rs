mod dexcom;
mod db;
mod worker;
mod keychain;
mod error;

use db::init_db;
use error::init_logger;
use keychain::get_password;
use db::get_setting;
use dexcom::Region;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let log_dir = app.path().app_log_dir()
                .expect("Failed to get log dir");
            init_logger(log_dir.to_str().unwrap());

            let db_path = app.path().app_data_dir()
                .expect("Failed to get app data dir")
                .join("glucotray.db");

            let db_path_str = db_path.to_str().unwrap().to_string();

            tauri::async_runtime::spawn(async move {
                let pool = init_db(&db_path_str).await
                    .expect("Failed to initialize database");

                let username = get_setting(&pool, "username").await
                    .unwrap_or(None);
                let region_str = get_setting(&pool, "region").await
                    .unwrap_or(None);

                if let (Some(username), Some(region_str)) = (username, region_str) {
                    let password = get_password(&username).unwrap_or_default();

                    let region = match region_str.as_str() {
                        "ous" => Region::Ous,
                        "jp" => Region::Jp,
                        _ => Region::Us,
                    };

                    worker::start_worker(pool, username, password, region).await;
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}