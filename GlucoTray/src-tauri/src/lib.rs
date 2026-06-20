mod dexcom;
mod db;
mod worker;
mod keychain;
mod error;
mod tray;
mod state;

pub use state::AppState;

use db::init_db;
use error::{init_logger, AppError};
use keychain::{get_password, save_credentials};
use db::{get_setting, set_setting};
use dexcom::{Region, DexcomClient};
use tauri::Manager;
use tray::TrayState;

const MMOL_TO_MGDL: f32 = 18.0182;

#[derive(serde::Serialize)]
struct SettingsData {
    username: String,
    region: String,
    unit: String,
    threshold_low_mgdl: i32,
    threshold_high_mgdl: i32,
    autostart: bool,
    color_critical_low: String,
    color_low: String,
    color_normal: String,
    color_high: String,
    color_very_high: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn error_code(e: &AppError) -> String {
    match e {
        AppError::InvalidCredentials => "InvalidCredentials".to_string(),
        AppError::NoSession => "NoSession".to_string(),
        AppError::NoReadings => "NoReadings".to_string(),
        AppError::Timeout => "Timeout".to_string(),
        AppError::RateLimit => "RateLimit".to_string(),
        AppError::SessionExpired => "SessionExpired".to_string(),
        AppError::NoInternetConnection => "NoInternetConnection".to_string(),
        AppError::KeychainError(_) => "KeychainError".to_string(),
        AppError::DbError(_) => "DbError".to_string(),
        AppError::NetworkError(_) => "NetworkError".to_string(),
        AppError::Unknown(_) => "Unknown".to_string(),
    }
}

async fn open_db(app: &tauri::AppHandle) -> Result<sqlx::SqlitePool, String> {
    let db_path = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("glucotray.db");

    let db_path_str = db_path.to_str().ok_or("Invalid db path")?.to_string();
    init_db(&db_path_str).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn validate_credentials(
    app: tauri::AppHandle,
    username: String,
    password: String,
    region: String,
) -> Result<(), String> {
    let r = match region.as_str() {
        "ous" => Region::Ous,
        "jp"  => Region::Jp,
        _     => Region::Us,
    };

    let mut client = DexcomClient::new(r);

    if let Err(e) = client.authenticate(&username, &password).await {
        e.log();
        return Err(error_code(&e));
    }

    if let Err(_) = save_credentials(&username, &password) {
        return Err("KeychainError".to_string());
    }

    let pool = open_db(&app).await?;
    set_setting(&pool, "username", &username).await.map_err(|e| e.to_string())?;

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
    let pool = open_db(&app).await?;

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

#[tauri::command]
async fn save_legal_acceptance(
    app: tauri::AppHandle,
    legal_version: String,
) -> Result<(), String> {
    let pool = open_db(&app).await?;

    set_setting(&pool, "privacy_accepted", "true").await.map_err(|e| e.to_string())?;
    set_setting(&pool, "privacy_version", &legal_version).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "terms_accepted", "true").await.map_err(|e| e.to_string())?;
    set_setting(&pool, "terms_version", &legal_version).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "disclaimer_accepted", "true").await.map_err(|e| e.to_string())?;
    set_setting(&pool, "disclaimer_version", &legal_version).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn read_legal_document(app: tauri::AppHandle, document: String, lang: String) -> Result<String, String> {
    let filename = format!("{}.{}.md", document, lang);

    let resource_path = app.path()
        .resolve(format!("legal/{}", filename), tauri::path::BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    std::fs::read_to_string(&resource_path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_wizard_status(app: tauri::AppHandle) -> Result<bool, String> {
    let pool = open_db(&app).await?;

    let wizard_done = get_setting(&pool, "wizard_done").await
        .map_err(|e| e.to_string())?
        .map(|v| v == "true")
        .unwrap_or(false);

    Ok(wizard_done)
}

#[tauri::command]
async fn get_settings(app: tauri::AppHandle) -> Result<SettingsData, String> {
    let pool = open_db(&app).await?;

    let username = get_setting(&pool, "username").await.map_err(|e| e.to_string())?.unwrap_or_default();
    let region = get_setting(&pool, "region").await.map_err(|e| e.to_string())?.unwrap_or_default();
    let unit = get_setting(&pool, "unit").await.map_err(|e| e.to_string())?.unwrap_or_else(|| "mgdl".to_string());

    let threshold_low_mgdl = get_setting(&pool, "threshold_low").await
        .map_err(|e| e.to_string())?
        .and_then(|v| v.parse().ok())
        .unwrap_or(70);

    let threshold_high_mgdl = get_setting(&pool, "threshold_high").await
        .map_err(|e| e.to_string())?
        .and_then(|v| v.parse().ok())
        .unwrap_or(180);

    let autostart = get_setting(&pool, "autostart").await
        .map_err(|e| e.to_string())?
        .map(|v| v == "true")
        .unwrap_or(false);

    let color_critical_low = get_setting(&pool, "color_critical_low").await.map_err(|e| e.to_string())?.unwrap_or_else(|| "#C62828".to_string());
    let color_low          = get_setting(&pool, "color_low").await.map_err(|e| e.to_string())?.unwrap_or_else(|| "#EF6C00".to_string());
    let color_normal       = get_setting(&pool, "color_normal").await.map_err(|e| e.to_string())?.unwrap_or_else(|| "#2E7D32".to_string());
    let color_high         = get_setting(&pool, "color_high").await.map_err(|e| e.to_string())?.unwrap_or_else(|| "#F9A825".to_string());
    let color_very_high    = get_setting(&pool, "color_very_high").await.map_err(|e| e.to_string())?.unwrap_or_else(|| "#D84315".to_string());

    Ok(SettingsData {
        username,
        region,
        unit,
        threshold_low_mgdl,
        threshold_high_mgdl,
        autostart,
        color_critical_low,
        color_low,
        color_normal,
        color_high,
        color_very_high,
    })
}

#[tauri::command]
async fn save_settings(
    app: tauri::AppHandle,
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
    let pool = open_db(&app).await?;

    set_setting(&pool, "unit",               &unit).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "threshold_low",      &threshold_low_mgdl.to_string()).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "threshold_high",     &threshold_high_mgdl.to_string()).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "autostart",          &autostart.to_string()).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "color_critical_low", &color_critical_low).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "color_low",          &color_low).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "color_normal",       &color_normal).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "color_high",         &color_high).await.map_err(|e| e.to_string())?;
    set_setting(&pool, "color_very_high",    &color_very_high).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(feature = "self-updater")]
#[tauri::command]
async fn check_for_update(app: tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_updater::UpdaterExt;

    let updater = app.updater().map_err(|e| e.to_string())?;

    match updater.check().await {
        Ok(Some(update)) => {
            update
                .download_and_install(|_chunk, _total| {}, || {})
                .await
                .map_err(|e| e.to_string())?;

            Ok("updated".to_string())
        }
        Ok(None) => Ok("up_to_date".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(not(feature = "self-updater"))]
#[tauri::command]
async fn check_for_update(_app: tauri::AppHandle) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        Ok("store_hint".to_string())
    }

    #[cfg(target_os = "linux")]
    {
        Ok("flatpak_hint".to_string())
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Ok("unsupported".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ));

    #[cfg(feature = "self-updater")]
    {
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
    }

    builder
        .manage(std::sync::Mutex::new(TrayState { update_available: false }))
        .manage(std::sync::Mutex::new(AppState { unit: "mgdl".to_string() }))
        .invoke_handler(tauri::generate_handler![
            greet,
            validate_credentials,
            save_wizard_data,
            restart_app,
            read_legal_document,
            save_legal_acceptance,
            get_wizard_status,
            get_settings,
            save_settings,
            check_for_update,
        ])
        .setup(|app| {
            let log_dir = app.path().app_log_dir()
                .expect("Failed to get log dir");
            init_logger(log_dir.to_str().unwrap());

            tray::setup_tray(app.handle())?;

            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = window_clone.hide();
                    }
                });
            }

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
                    let unit = get_setting(&pool, "unit").await
                        .unwrap_or(None)
                        .unwrap_or_else(|| "mgdl".to_string());

                    let autostart = get_setting(&pool, "autostart").await
                        .unwrap_or(None)
                        .map(|v| v == "true")
                        .unwrap_or(false);

                    {
                        let state = app_handle.state::<std::sync::Mutex<AppState>>();
                        let mut s = state.lock().unwrap();
                        s.unit = unit;
                    }

                    {
                        use tauri_plugin_autostart::ManagerExt;
                        let autostart_manager = app_handle.autolaunch();
                        if autostart {
                            let _ = autostart_manager.enable();
                        } else {
                            let _ = autostart_manager.disable();
                        }
                    }

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