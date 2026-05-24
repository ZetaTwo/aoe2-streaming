use anyhow::{anyhow, Context, Result};
use hyper_util::client::legacy::connect::HttpConnector;
use yup_oauth2::{
    authenticator::{ApplicationDefaultCredentialsTypes, Authenticator},
    hyper_rustls::HttpsConnector,
    ApplicationDefaultCredentialsAuthenticator, ApplicationDefaultCredentialsFlowOpts,
};

use crate::parse::BatchGetResponse;

const SCOPES: &[&str] = &["https://www.googleapis.com/auth/spreadsheets.readonly"];

pub struct SheetsClient {
    auth: Authenticator<HttpsConnector<HttpConnector>>,
    http: reqwest::Client,
}

impl SheetsClient {
    pub async fn new() -> Result<Self> {
        let opts = ApplicationDefaultCredentialsFlowOpts::default();
        let auth = match ApplicationDefaultCredentialsAuthenticator::builder(opts).await {
            ApplicationDefaultCredentialsTypes::InstanceMetadata(builder) => builder
                .build()
                .await
                .context("building GCE-metadata authenticator")?,
            ApplicationDefaultCredentialsTypes::ServiceAccount(builder) => builder
                .build()
                .await
                .context("building service-account authenticator")?,
        };
        let http = reqwest::Client::builder()
            .build()
            .context("building reqwest client")?;
        Ok(Self { auth, http })
    }

    pub async fn batch_get(&self, sheet_id: &str, ranges: &[String]) -> Result<BatchGetResponse> {
        let token = self
            .auth
            .token(SCOPES)
            .await
            .context("fetching Sheets access token")?;
        let bearer = token
            .token()
            .ok_or_else(|| anyhow!("ADC returned an empty access token"))?;

        let url =
            format!("https://sheets.googleapis.com/v4/spreadsheets/{sheet_id}/values:batchGet");
        let query: Vec<(&str, &str)> = ranges.iter().map(|r| ("ranges", r.as_str())).collect();

        let resp = self
            .http
            .get(&url)
            .query(&query)
            .bearer_auth(bearer)
            .send()
            .await
            .context("calling Sheets API")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Sheets API returned {}: {}",
                status,
                body.chars().take(500).collect::<String>()
            ));
        }
        let body = resp
            .json::<BatchGetResponse>()
            .await
            .context("decoding Sheets API response")?;
        Ok(body)
    }
}
