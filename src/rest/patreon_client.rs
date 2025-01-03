use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, IntoUrl, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::{Error, Result};

pub struct PatreonClientBuilder {
    api_key: String,
    client_id: String,
}

impl PatreonClientBuilder {
    pub fn new(api_key: impl Into<String>, client_id: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client_id: client_id.into(),
        }
    }

    pub fn build(self) -> Result<PatreonClient> {
        PatreonClient::new(&self.api_key)
    }
}

pub struct PatreonClient {
    client: Client,
}

impl PatreonClient {
    pub fn new(api_key: &str) -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        Ok(PatreonClient { client })
    }

    pub async fn get<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T> {
        let reqwest = self.client.get(url);
        self.process_response::<T>(reqwest).await
    }

    pub async fn process_response<T: DeserializeOwned>(
        &self,
        reqwest: RequestBuilder,
    ) -> Result<T> {
        let response = reqwest.send().await?;

        if let Some(hv) = response.headers().get("Content-Type") {
            if !hv
                .to_str()
                .map(|s| s.starts_with("application/json"))
                .unwrap_or(false)
            {
                return Err(Error::InvalidContentType(hv.to_owned()));
            }
        }

        let status = response.status();

        if status.is_success() {
            Ok(response.json::<T>().await?)
        } else {
            let text = response.text().await?;
            Err(Error::UnexpectedResponse(status, text))
        }
    }
}
