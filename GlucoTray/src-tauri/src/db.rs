use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn init_db(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}?mode=rwc", db_path))
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS readings (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            value       INTEGER NOT NULL,
            trend       TEXT NOT NULL,
            timestamp   TEXT NOT NULL,
            is_valid    INTEGER NOT NULL DEFAULT 1,
            created_at  TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS settings (
            key     TEXT PRIMARY KEY NOT NULL,
            value   TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn insert_reading(
    pool: &SqlitePool,
    value: i32,
    trend: &str,
    timestamp: &str,
    is_valid: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO readings (value, trend, timestamp, is_valid)
         VALUES (?, ?, ?, ?)"
    )
    .bind(value)
    .bind(trend)
    .bind(timestamp)
    .bind(is_valid as i32)
    .execute(pool)
    .await?;

    sqlx::query(
        "DELETE FROM readings
         WHERE created_at < datetime('now', '-30 minutes')"
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_latest_reading(
    pool: &SqlitePool,
) -> Result<Option<(i32, String, bool)>, sqlx::Error> {
    let row = sqlx::query_as::<_, (i32, String, i32)>(
        "SELECT value, trend, is_valid FROM readings
         ORDER BY created_at DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(value, trend, is_valid)| (value, trend, is_valid != 0)))
}

pub async fn set_setting(
    pool: &SqlitePool,
    key: &str,
    value: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO settings (key, value)
         VALUES (?, ?)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value"
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_setting(
    pool: &SqlitePool,
    key: &str,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query_as::<_, (String,)>(
        "SELECT value FROM settings WHERE key = ?"
    )
    .bind(key)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(value,)| value))
}