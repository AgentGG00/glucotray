use sqlx::SqlitePool;
use tokio::time::{sleep, Duration};
use crate::dexcom::{DexcomClient, Region};
use crate::db::{insert_reading};

const POLL_INTERVAL_SECS: u64 = 150;
const MAX_FAILURES: u8 = 8;

pub async fn start_worker(
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
                }
            }
        }

        sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
    }
}