use serde_json::Value;
use url::Url;

use crate::Result;

use super::includes::IdentityInclude;
use super::PatreonClient;

const BASE_URI: &str = "https://www.patreon.com/api/oauth2/v2";

impl PatreonClient {
    pub async fn identity(&self, include: Option<IdentityInclude>) -> Result<Value> {
        let mut url = Url::parse(BASE_URI)?;
        url.set_path("/identity");
        // url.query_pairs_mut().append_pair("fields[user]", "");

        if let Some(include) = include {
            url.query_pairs_mut()
                .append_pair("include", &include.to_string());
        }
        self.get(format!("{}/identity", BASE_URI)).await
    }

    // https://www.patreon.com/api/oauth2/v2/identity?fields[user]=about,created,email,first_name,full_name,image_url,last_name,social_connections,thumb_url,url,vanity

    pub async fn campaigns(&self) -> Result<Vec<Campaign>> {
        self.get(format!("{}/campaigns", BASE_URI)).await
    }

    pub async fn campaign(&self, campaign_id: u64) -> Result<Campaign> {
        self.get(format!("{}/campaigns/{}", BASE_URI, campaign_id))
            .await
    }

    pub async fn campaign_members(&self, campaign_id: u64) -> Result<Vec<CampaignMember>> {
        self.get(format!("{}/campaigns/{}/members", BASE_URI, campaign_id))
            .await
    }

    pub async fn campaign_member(
        &self,
        campaign_id: u64,
        member_id: u64,
    ) -> Result<CampaignMember> {
        self.get(format!(
            "{}/campaigns/{}/members/{}",
            BASE_URI, campaign_id, member_id
        ))
        .await
    }

    pub async fn campaign_posts(&self, campaign_id: u64) -> Result<Vec<CampaignPost>> {
        self.get(format!("{}/campaigns/{}/posts", BASE_URI, campaign_id))
            .await
    }

    pub async fn post(&self, post_id: u64) -> Result<CampaignPost> {
        self.get(format!("{}/posts/{}", BASE_URI, post_id)).await
    }
}
