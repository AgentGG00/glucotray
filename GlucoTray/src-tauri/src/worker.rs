use sqlx::SqlitePool;
use tokio::time::{sleep, Duration};
use tauri::AppHandle;
use crate::dexcom::{DexcomClient, Region};
use crate::db::{insert_reading, get_setting};
use crate::tray::update_tray;

const POLL_INTERVAL_SECS: u64 = 150;
const MAX_FAILURES: u8 = 8;

fn color_for_value(value: i32, threshold_low: i32, threshold_high: i32) -> &'static str {
    if value <= 0                    { "#6B7280" }
    else if value < threshold_low    { "#C62828" }
    else if value > threshold_high   { "#D84315" }
    else                             { "#2E7D32" }
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

        match client.get_readings(&password).await {
            Ok(readings) => {
                if let Some(reading) = readings.into_iter().next() {
                    failure_count = 0;
                    na_written = false;

                    let _ = insert_reading(
                        &pool,
                        reading.value as i32,
                        &reading.trend,
                        &reading.timestamp,
                        true,
                    )
                    .await;

                    let color = color_for_value(reading.value as i32, threshold_low, threshold_high);
                    update_tray(&app, reading.value as i32, &reading.trend, color);
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