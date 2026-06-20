use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::error::AppError;

const APP_ID: &str = "d89443d2-327c-4a6f-89e5-496bbb0317db";

const BASE_URL_US: &str = "https://share2.dexcom.com/ShareWebServices/Services";
const BASE_URL_OUS: &str = "https://shareous1.dexcom.com/ShareWebServices/Services";
const BASE_URL_JP: &str = "https://share.dexcom.jp/ShareWebServices/Services";
const DNS_CHECK_HOST: &str = "8.8.8.8:53";

#[derive(Debug, Serialize, Deserialize)]
pub struct GlucoseReading {
    #[serde(rename = "Value")]
    pub value: u32,
    #[serde(rename = "Trend")]
    pub trend: String,
    #[serde(rename = "WT")]
    pub timestamp: String,
}

pub enum Region {
    Us,
    Ous,
    Jp,
}

impl Region {
    fn base_url(&self) -> &str {
        match self {
            Region::Us => BASE_URL_US,
            Region::Ous => BASE_URL_OUS,
            Region::Jp => BASE_URL_JP,
        }
    }
}

pub struct DexcomClient {
    client: Client,
    base_url: String,
    account_id: Option<String>,
    session_id: Option<String>,
}

impl DexcomClient {
    pub fn new(region: Region) -> Self {
        Self {
            client: Client::new(),
            base_url: region.base_url().to_string(),
            account_id: None,
            session_id: None,
        }
    }

    async fn check_internet_connection(&self) -> Result<(), AppError> {
        match tokio::time::timeout(
            std::time::Duration::from_secs(3),
            tokio::net::TcpStream::connect(DNS_CHECK_HOST),
        )
        .await
        {
            Ok(Ok(_)) => Ok(()),
            _ => Err(AppError::NoInternetConnection),
        }
    }

    pub async fn authenticate(&mut self, username: &str, password: &str) -> Result<(), AppError> {
        self.check_internet_connection().await?;

        let url = format!("{}/General/AuthenticatePublisherAccount", self.base_url);

        let body = serde_json::json!({
            "accountName": username,
            "password": password,
            "applicationId": APP_ID,
        });

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        match response.status().as_u16() {
            200..=299 => {}
            401 | 403 => return Err(AppError::InvalidCredentials),
            429 => return Err(AppError::RateLimit),
            _ => return Err(AppError::Unknown(format!("Auth failed with status {}", response.status()))),
        }

        let account_id: String = response
            .text()
            .await?
            .trim_matches('"')
            .to_string();

        self.account_id = Some(account_id);
        self.fetch_session(password).await
    }

    async fn fetch_session(&mut self, password: &str) -> Result<(), AppError> {
        let account_id = self.account_id.clone().ok_or(AppError::NoSession)?;
        let url = format!("{}/General/LoginPublisherAccountById", self.base_url);

        let body = serde_json::json!({
            "accountId": account_id,
            "password": password,
            "applicationId": APP_ID,
        });

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        match response.status().as_u16() {
            200..=299 => {}
            401 | 403 => return Err(AppError::InvalidCredentials),
            429 => return Err(AppError::RateLimit),
            _ => return Err(AppError::NoSession),
        }

        let session_id: String = response
            .text()
            .await?
            .trim_matches('"')
            .to_string();

        self.session_id = Some(session_id);
        Ok(())
    }

    pub async fn get_readings(&mut self, password: &str) -> Result<Vec<GlucoseReading>, AppError> {
        let session_id = self.session_id.clone().ok_or(AppError::NoSession)?;
        let url = format!("{}/Publisher/ReadPublisherLatestGlucoseValues", self.base_url);

        let body = serde_json::json!({
            "sessionId": session_id,
            "minutes": 10,
            "maxCount": 1,
        });

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if response.status().as_u16() == 500 {
            self.fetch_session(password).await?;
            return Err(AppError::SessionExpired);
        }

        match response.status().as_u16() {
            200..=299 => {}
            429 => return Err(AppError::RateLimit),
            _ => return Err(AppError::Unknown(format!("Readings fetch failed with status {}", response.status()))),
        }

        let readings: Vec<GlucoseReading> = response.json().await?;

        if readings.is_empty() {
            return Err(AppError::NoReadings);
        }

        Ok(readings)
    }
}