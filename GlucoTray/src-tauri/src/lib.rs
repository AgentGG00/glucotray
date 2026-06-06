mod dexcom;
mod db;
mod worker;
mod keychain;
mod error;
mod tray;

use db::{init_db, mgdl_to_mmol};
use error::init_logger;
use keychain::{get_password, save_credentials};
use db::{get_setting, set_setting};
use dexcom::{Region, DexcomClient};
use tauri::Manager;
use tray::TrayState;

const MMOL_TO_MGDL: f32 = 18.0182;

fn mmol_to_mgdl(mmol: f32) -> i32 {
    (mmol * MMOL_TO_MGDL).round() as i32
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn validate_credentials(username: String, password: String, region: String) -> Result<(), String> {
    let r = match region.as_str() {
        "ous" => Region::Ous,
        "jp"  => Region::Jp,
        _     => Region::Us,
    };

    let mut client = DexcomClient::new(r);
    client.authenticate(&username, &password).await?;
    save_credentials(&username, &password).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn save_wizard_data(
    app: tauri::AppHandle,
    username: String,
    region: String,
    sensor: String,
    unit: String,
    threshold_low_mgdl: i32,
    threshold_high_mgdl: i32,
    autostart: bool,
    color_critical_low: String,
    color_low: String,
    color_normal: String,
    color_high: String,
    color_very_high: String,
) -> Result<(), String> {
    let db_path = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("glucotray.db");

    let db_path_str = db_path.to_str().ok_or("Invalid db path")?.to_string();
    let pool = init_db(&db_path_str).await.map_err(|e| e.to_string())?;

    set_setting(&pool, "username",           &username).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "region",             &region).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "sensor",             &sensor).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "unit",               &unit).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "threshold_low",      &threshold_low_mgdl.to_string()).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "threshold_high",     &threshold_high_mgdl.to_string()).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "autostart",          &autostart.to_string()).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "color_critical_low", &color_critical_low).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "color_low",          &color_low).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "color_normal",       &color_normal).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "color_high",         &color_high).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "color_very_high",    &color_very_high).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "wizard_done",        "true").await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn restart_app(app: tauri::AppHandle) {
    app.restart();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .manage(std::sync::Mutex::new(TrayState { update_available: false }))
        .invoke_handler(tauri::generate_handler![
            greet,
            validate_credentials,
            save_wizard_data,
            restart_app,
        ])
        .setup(|app| {
            let log_dir = app.path().app_log_dir()
                .expect("Failed to get log dir");
            init_logger(log_dir.to_str().unwrap());

            tray::setup_tray(app.handle())?;

            let db_path = app.path().app_data_dir()
                .expect("Failed to get app data dir")
                .join("glucotray.db");
            let db_path_str = db_path.to_str().unwrap().to_string();
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let pool = init_db(&db_path_str).await
                    .expect("Failed to initialize database");

                let wizard_done = get_setting(&pool, "wizard_done").await
                    .unwrap_or(None)
                    .map(|v| v == "true")
                    .unwrap_or(false);

                if wizard_done {
                    let username = get_setting(&pool, "username").await.unwrap_or(None);
                    let region_str = get_setting(&pool, "region").await.unwrap_or(None);

                    if let (Some(username), Some(region_str)) = (username, region_str) {
                        let password = get_password(&username).unwrap_or_default();

                        let region = match region_str.as_str() {
                            "ous" => Region::Ous,
                            "jp"  => Region::Jp,
                            _     => Region::Us,
                        };

                        let first_start = get_setting(&pool, "tray_hint_shown").await
                            .unwrap_or(None)
                            .is_none();

                        if first_start {
                            let _ = set_setting(&pool, "tray_hint_shown", "true").await;
                            tauri_plugin_notification::NotificationExt::notification(&app_handle)
                                .builder()
                                .title("GlucoTray läuft")
                                .body("Damit der Blutzuckerwert immer sichtbar ist, pinne das Icon in der Taskleiste.")
                                .show()
                                .ok();
                        }

                        worker::start_worker(app_handle, pool, username, password, region).await;
                    }
                } else {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}