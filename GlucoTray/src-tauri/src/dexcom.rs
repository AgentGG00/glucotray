use reqwest::Client;
use serde::{Deserialize, Serialize};

const APP_ID: &str = "d89443d2-327c-4a6f-89e5-496bbb0317db";

const BASE_URL_US: &str = "https://share2.dexcom.com/ShareWebServices/Services";
const BASE_URL_OUS: &str = "https://shareous1.dexcom.com/ShareWebServices/Services";
const BASE_URL_JP: &str = "https://share.dexcom.jp/ShareWebServices/Services";

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

    pub async fn authenticate(&mut self, username: &str, password: &str) -> Result<(), String> {
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
            .await
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            return Err("Invalid credentials".to_string());
        }

        let account_id: String = response
            .text()
            .await
            .map_err(|e| e.to_string())?
            .trim_matches('"')
            .to_string();

        self.account_id = Some(account_id);
        self.fetch_session(password).await
    }

    async fn fetch_session(&mut self, password: &str) -> Result<(), String> {
        let account_id = self.account_id.clone().ok_or("No account ID")?;
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
            .await
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            return Err("Failed to create session".to_string());
        }

        let session_id: String = response
            .text()
            .await
            .map_err(|e| e.to_string())?
            .trim_matches('"')
            .to_string();

        self.session_id = Some(session_id);
        Ok(())
    }

    pub async fn get_readings(&mut self, password: &str) -> Result<Vec<GlucoseReading>, String> {
        let session_id = self.session_id.clone().ok_or("No session ID")?;
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
            .await
            .map_err(|e| e.to_string())?;

        if response.status().as_u16() == 500 {
            self.fetch_session(password).await?;
            return Err("Session expired, retry".to_string());
        }

        if !response.status().is_success() {
            return Err("Failed to fetch readings".to_string());
        }

        let readings: Vec<GlucoseReading> = response
            .json()
            .await
            .map_err(|e| e.to_string())?;

        Ok(readings)
    }
}