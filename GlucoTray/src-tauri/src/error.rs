use tracing::{error, warn};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

#[derive(Debug)]
pub enum AppError {
    InvalidCredentials,
    NoSession,
    NoReadings,
    Timeout,
    RateLimit,
    SessionExpired,
    NoInternetConnection,
    KeychainError(String),
    DbError(String),
    NetworkError(String),
    Unknown(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidCredentials => write!(f, "Username or password incorrect"),
            AppError::NoSession => write!(f, "No active Share session found"),
            AppError::NoReadings => write!(f, "No current readings found"),
            AppError::Timeout => write!(f, "Connection to Dexcom API failed"),
            AppError::RateLimit => write!(f, "Too many requests, please wait"),
            AppError::SessionExpired => write!(f, "Session expired, reconnecting"),
            AppError::NoInternetConnection => write!(f, "No internet connection detected"),
            AppError::KeychainError(msg) => write!(f, "Keychain error: {}", msg),
            AppError::DbError(msg) => write!(f, "Database error: {}", msg),
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AppError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl AppError {
    pub fn log(&self) {
        match self {
            AppError::InvalidCredentials | AppError::NoSession => {
                error!(error = %self, "Authentication error");
            }
            AppError::RateLimit | AppError::Timeout | AppError::NetworkError(_) | AppError::NoInternetConnection => {
                warn!(error = %self, "Network error");
            }
            AppError::DbError(_) | AppError::KeychainError(_) => {
                error!(error = %self, "System error");
            }
            AppError::SessionExpired | AppError::NoReadings => {
                warn!(error = %self, "Session or reading warning");
            }
            AppError::Unknown(_) => {
                error!(error = %self, "Unknown error");
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::DbError(e.to_string())
    }
}

impl From<keyring::Error> for AppError {
    fn from(e: keyring::Error) -> Self {
        AppError::KeychainError(e.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            AppError::Timeout
        } else if e.is_connect() {
            AppError::NoInternetConnection
        } else {
            AppError::NetworkError(e.to_string())
        }
    }
}

pub fn init_logger(log_dir: &str) {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "glucotray.log");

    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(fmt::layer().with_writer(file_appender).with_ansi(false))
        .with(fmt::layer().with_writer(std::io::stdout))
        .init();
}