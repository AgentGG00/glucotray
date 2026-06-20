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
use tracing::{info, error};

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
    tray_icon_size: u32,
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
    let data_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?;

    std::fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;

    let db_path = data_dir.join("glucotray.db");
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
    set_setting(&pool, "tray_icon_size",     "32").await.map_err(|e| e.to_string())?;
    set_setting(&pool, "wizard_done",        "true").await.map_err(|e| e.to_string())?;

    info!("save_wizard_data: wizard completed, wizard_done set to true");

    Ok(())
}

#[tauri::command]
async fn restart_app(app: tauri::AppHandle) {
    info!("restart_app: restarting application");
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

    info!(version = %legal_version, "save_legal_acceptance: legal documents accepted");

    Ok(())
}

#[tauri::command]
fn read_legal_document(app: tauri::AppHandle, document: String, lang: String) -> Result<String, String> {
    let filename = format!("{}.{}.md", document, lang);

    let resource_path = app.path()
        .resolve(format!("legal/{}", filename), tauri::path::BaseDirectory::Resource)
        .map_err(|e| {
            error!(filename = %filename, error = %e, "read_legal_document: failed to resolve resource path");
            e.to_string()
        })?;

    std::fs::read_to_string(&resource_path).map_err(|e| {
        error!(path = %resource_path.display(), error = %e, "read_legal_document: failed to read file");
        e.to_string()
    })
}

#[tauri::command]
async fn get_wizard_status(app: tauri::AppHandle) -> Result<bool, String> {
    let pool = open_db(&app).await?;

    let wizard_done = get_setting(&pool, "wizard_done").await
        .map_err(|e| e.to_string())?
        .map(|v| v == "true")
        .unwrap_or(false);

    info!(wizard_done = wizard_done, "get_wizard_status: queried");

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

    let tray_icon_size = get_setting(&pool, "tray_icon_size").await
        .map_err(|e| e.to_string())?
        .and_then(|v| v.parse().ok())
        .unwrap_or(32);

    info!("get_settings: loaded settings for settings window");

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
        tray_icon_size,
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

    info!("save_settings: settings window changes saved");

    Ok(())
}

#[tauri::command]
async fn set_tray_icon_size(app: tauri::AppHandle, size: u32) -> Result<(), String> {
    {
        let state = app.state::<std::sync::Mutex<AppState>>();
        let mut s = state.lock().unwrap();
        s.tray_icon_size = size;
    }

    let pool = open_db(&app).await?;
    set_setting(&pool, "tray_icon_size", &size.to_string()).await.map_err(|e| e.to_string())?;

    tray::refresh_tray_icon(&app);

    info!(size = size, "set_tray_icon_size: tray icon size updated live");

    Ok(())
}

#[cfg(feature = "self-updater")]
#[tauri::command]
async fn check_for_update(app: tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_updater::UpdaterExt;

    let updater = app.updater().map_err(|e| e.to_string())?;

    match updater.check().await {
        Ok(Some(update)) => {
            info!(version = %update.version, "check_for_update: update found, downloading and installing");
            update
                .download_and_install(|_chunk, _total| {}, || {})
                .await
                .map_err(|e| e.to_string())?;

            Ok("updated".to_string())
        }
        Ok(None) => {
            info!("check_for_update: already up to date");
            Ok("up_to_date".to_string())
        }
        Err(e) => {
            error!(error = %e, "check_for_update: failed");
            Err(e.to_string())
        }
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
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            info!("single_instance: second launch attempt detected, focusing existing window");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ));

    #[cfg(feature = "self-updater")]
    {
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
    }

    builder
        .manage(std::sync::Mutex::new(TrayState {
            update_available: false,
            last_value_mgdl: 0,
            last_trend: "Flat".to_string(),
            last_color: "#6B7280".to_string(),
        }))
        .manage(std::sync::Mutex::new(AppState { unit: "mgdl".to_string(), tray_icon_size: 32 }))
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
            set_tray_icon_size,
            check_for_update,
        ])
        .setup(|app| {
            let log_dir = app.path().app_log_dir()
                .expect("Failed to get log dir");
            let log_dir_str = log_dir.to_str().unwrap().to_string();
            init_logger(&log_dir_str);

            info!(path = %log_dir_str, "setup: logger initialized");

            let is_autostart = std::env::args().any(|arg| arg == "--autostart");
            info!(is_autostart = is_autostart, "setup: launch mode determined");

            tray::setup_tray(app.handle())?;
            info!("setup: tray icon created");

            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = window_clone.hide();
                        info!("window_event: close requested, hiding window instead of quitting");
                    }
                });
                info!("setup: window close interceptor registered");
            } else {
                error!("setup: could not find webview window 'main' to attach close interceptor");
            }

            let data_dir = match app.path().app_data_dir() {
                Ok(dir) => dir,
                Err(e) => {
                    error!(error = %e, "setup: failed to resolve app data dir");
                    panic!("Failed to get app data dir: {}", e);
                }
            };

            if let Err(e) = std::fs::create_dir_all(&data_dir) {
                error!(path = %data_dir.display(), error = %e, "setup: failed to create app data dir");
                panic!("Failed to create app data dir: {}", e);
            }
            info!(path = %data_dir.display(), "setup: app data dir created/verified");

            let db_path = data_dir.join("glucotray.db");
            let db_path_str = db_path.to_str().unwrap().to_string();
            info!(path = %db_path_str, "setup: resolved database path");

            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                info!("setup_async: background initialization task started");

                let pool = match init_db(&db_path_str).await {
                    Ok(pool) => {
                        info!(path = %db_path_str, "setup_async: database initialized successfully");
                        pool
                    }
                    Err(e) => {
                        error!(path = %db_path_str, error = %e, "setup_async: database initialization FAILED");
                        return;
                    }
                };

                let wizard_done = match get_setting(&pool, "wizard_done").await {
                    Ok(v) => v.map(|v| v == "true").unwrap_or(false),
                    Err(e) => {
                        error!(error = %e, "setup_async: failed to read wizard_done setting");
                        false
                    }
                };

                info!(wizard_done = wizard_done, "setup_async: wizard status determined");

                if wizard_done {
                    let username = get_setting(&pool, "username").await.unwrap_or(None);
                    let region_str = get_setting(&pool, "region").await.unwrap_or(None);
                    let unit = get_setting(&pool, "unit").await
                        .unwrap_or(None)
                        .unwrap_or_else(|| "mgdl".to_string());

                    let tray_icon_size: u32 = get_setting(&pool, "tray_icon_size").await
                        .unwrap_or(None)
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(32);

                    let autostart = get_setting(&pool, "autostart").await
                        .unwrap_or(None)
                        .map(|v| v == "true")
                        .unwrap_or(false);

                    {
                        let state = app_handle.state::<std::sync::Mutex<AppState>>();
                        let mut s = state.lock().unwrap();
                        s.unit = unit;
                        s.tray_icon_size = tray_icon_size;
                    }
                    info!(tray_icon_size = tray_icon_size, "setup_async: AppState populated from DB");

                    {
                        use tauri_plugin_autostart::ManagerExt;
                        let autostart_manager = app_handle.autolaunch();
                        if autostart {
                            let _ = autostart_manager.enable();
                            info!("setup_async: autostart enabled");
                        } else {
                            let _ = autostart_manager.disable();
                            info!("setup_async: autostart disabled");
                        }
                    }

                    if !is_autostart {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            match window.show() {
                                Ok(_) => info!("setup_async: manual launch detected, main window shown (settings)"),
                                Err(e) => error!(error = %e, "setup_async: failed to show main window on manual launch"),
                            }
                        }
                    } else {
                        info!("setup_async: autostart launch detected, keeping main window hidden");
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
                            info!("setup_async: first-start tray hint notification shown");
                        }

                        info!(username = %username, region = %region_str, "setup_async: starting worker");
                        worker::start_worker(app_handle, pool, username, password, region).await;
                    } else {
                        error!("setup_async: wizard_done is true but username/region missing in DB, cannot start worker");
                    }
                } else {
                    info!("setup_async: wizard not done, showing main window for wizard");
                    if let Some(window) = app_handle.get_webview_window("main") {
                        match window.show() {
                            Ok(_) => info!("setup_async: main window shown successfully"),
                            Err(e) => error!(error = %e, "setup_async: failed to show main window"),
                        }
                    } else {
                        error!("setup_async: could not find webview window 'main' to show");
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}