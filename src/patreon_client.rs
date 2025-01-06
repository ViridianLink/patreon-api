use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Response};
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
        let response = Self::validate_content_type(reqwest).await.unwrap();
        let text = response.text().await.unwrap();
        match serde_json::from_str::<T>(&text) {
            Ok(json) => Ok(json),
            Err(e) => {
                #[cfg(test)]
                std::fs::write("error.json", text).unwrap();
                Err(e.into())
            }
        }
    }

    async fn validate_content_type(reqwest: RequestBuilder) -> Result<Response> {
        let response = reqwest.send().await?;

        if let Some(hv) = response.headers().get("Content-Type") {
            if !hv
                .to_str()
                .map(|s| {
                    s.starts_with("application/json") || s.starts_with("application/vnd.api+json")
                })
                .unwrap_or(false)
            {
                return Err(Error::InvalidContentType(hv.to_owned()));
            }
        }

        Ok(response)
    }
}
