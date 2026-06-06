use sqlx::SqlitePool;
use tokio::time::{sleep, Duration};
use tauri::AppHandle;
use crate::dexcom::{DexcomClient, Region};
use crate::db::{insert_reading, get_setting};
use crate::tray::{update_tray, resolve_color, ColorScheme};

const POLL_INTERVAL_SECS: u64 = 150;
const MAX_FAILURES: u8 = 8;

async fn load_color_scheme(pool: &SqlitePool) -> ColorScheme {
    ColorScheme {
        critical_low: get_setting(pool, "color_critical_low").await.unwrap_or(None).unwrap_or_else(|| "#C62828".to_string()),
        low:          get_setting(pool, "color_low").await.unwrap_or(None).unwrap_or_else(|| "#EF6C00".to_string()),
        normal:       get_setting(pool, "color_normal").await.unwrap_or(None).unwrap_or_else(|| "#2E7D32".to_string()),
        high:         get_setting(pool, "color_high").await.unwrap_or(None).unwrap_or_else(|| "#F9A825".to_string()),
        very_high:    get_setting(pool, "color_very_high").await.unwrap_or(None).unwrap_or_else(|| "#D84315".to_string()),
    }
}

pub async fn start_worker(
    app: AppHandle,
    pool: SqlitePool,
    username: String,
    password: String,
    region: Region,
) {
    let mut client = DexcomClient::new(region);
    let mut failure_count: u8 = 0;
    let mut na_written = false;

    if let Err(_) = client.authenticate(&username, &password).await {
        failure_count = MAX_FAILURES;
    }

    loop {
        let threshold_low: i32 = get_setting(&pool, "threshold_low").await
            .unwrap_or(None)
            .and_then(|v| v.parse().ok())
            .unwrap_or(70);

        let threshold_high: i32 = get_setting(&pool, "threshold_high").await
            .unwrap_or(None)
            .and_then(|v| v.parse().ok())
            .unwrap_or(180);

        let colors = load_color_scheme(&pool).await;

        match client.get_readings(&password).await {
            Ok(readings) => {
                if let Some(reading) = readings.into_iter().next() {
                    failure_count = 0;
                    na_written = false;

                    let value_mgdl = reading.value as i32;

                    let _ = insert_reading(
                        &pool,
                        value_mgdl,
                        &reading.trend,
                        &reading.timestamp,
                        true,
                    )
                    .await;

                    let color = resolve_color(value_mgdl, &reading.trend, threshold_low, threshold_high, &colors);
                    update_tray(&app, value_mgdl, &reading.trend, &color);
                }
            }
            Err(_) => {
                failure_count += 1;

                if failure_count >= MAX_FAILURES && !na_written {
                    let _ = insert_reading(
                        &pool,
                        0,
                        "N/A",
                        &chrono::Utc::now().to_rfc3339(),
                        false,
                    )
                    .await;
                    na_written = true;

                    update_tray(&app, 0, "N/A", "#6B7280");
                }
            }
        }

        sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
    }
}