use std::sync::Arc;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{Error, Result};

pub struct PatreonOAuth {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    client: Arc<Client>,
}

impl PatreonOAuth {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
            client: Arc::new(Client::new()),
        }
    }

    pub fn authorization_url(&self, scope: &str, state: &str) -> Url {
        let mut url = Url::parse("https://www.patreon.com/oauth2/authorize").unwrap();

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("scope", scope)
            .append_pair("state", state)
            .finish();

        url
    }

    pub async fn tokens(&self, code: &str) -> Result<TokensResponse> {
        self.parse_request(&[
            ("code", code),
            ("grant_type", "authorization_code"),
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("redirect_uri", self.redirect_uri.as_str()),
        ])
        .await
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokensResponse> {
        self.parse_request(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
        ])
        .await
    }

    async fn parse_request(&self, params: &[(&str, &str)]) -> Result<TokensResponse> {
        let url = Url::parse("https://www.patreon.com/api/oauth2/token").unwrap();
        let res = self.client.post(url).form(params).send().await.unwrap();
        if res.status().is_success() {
            Ok(res.json().await.unwrap())
        } else {
            Err(Error::UnexpectedResponse(
                res.status(),
                res.text().await.unwrap(),
            ))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TokensResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub scope: String,
    pub refresh_token: String,
    pub version: String,
}
