pub mod endpoints;
pub mod error;
pub mod pagination_options;
pub mod patreon_client;
pub mod types;
pub use error::Error;
use error::Result;

#[cfg(test)]
mod tests {
    use crate::patreon_client::PatreonClientBuilder;
    use crate::types::includes::{CampaignInclude, IdentityInclude, MemberInclude, PostInclude};

    const API_KEY: &str = "";
    const CLIENT_ID: &str = "co3TJ3lwqHN5WSVuIBiDNhfQv28V4FR-z6g-_fIogDzj_Um09DoWLGE5rvAJeTQd";

    #[tokio::test]
    async fn test_identity() {
        let client = PatreonClientBuilder::new(API_KEY, CLIENT_ID)
            .build()
            .unwrap();

        client.identity(IdentityInclude::empty()).await.unwrap();

        client.identity(IdentityInclude::all()).await.unwrap();
    }

    #[tokio::test]
    async fn test_campaigns() {
        let client = PatreonClientBuilder::new(API_KEY, CLIENT_ID)
            .build()
            .unwrap();

        client.campaigns(CampaignInclude::empty()).await.unwrap();

        client.campaigns(CampaignInclude::all()).await.unwrap();
    }

    #[tokio::test]
    async fn test_campaign() {
        const CAMPAIGN_ID: u64 = 5167485;

        let client = PatreonClientBuilder::new(API_KEY, CLIENT_ID)
            .build()
            .unwrap();

        client
            .campaign(CAMPAIGN_ID, CampaignInclude::empty())
            .await
            .unwrap();

        client
            .campaign(CAMPAIGN_ID, CampaignInclude::all())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_campaign_members() {
        const CAMPAIGN_ID: u64 = 5167485;

        let client = PatreonClientBuilder::new(API_KEY, CLIENT_ID)
            .build()
            .unwrap();

        client
            .campaign_members(CAMPAIGN_ID, MemberInclude::empty(), None)
            .await
            .unwrap();

        client
            .campaign_members(CAMPAIGN_ID, MemberInclude::all(), None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_member() {
        const MEMBER_ID: &str = "00244c13-b7cf-4e21-b965-854d968fb603";

        let client = PatreonClientBuilder::new(API_KEY, CLIENT_ID)
            .build()
            .unwrap();

        client
            .member(MEMBER_ID, MemberInclude::empty())
            .await
            .unwrap();

        client
            .member(MEMBER_ID, MemberInclude::all())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_campaign_posts() {
        const CAMPAIGN_ID: u64 = 5167485;

        let client = PatreonClientBuilder::new(API_KEY, CLIENT_ID)
            .build()
            .unwrap();

        client
            .campaign_posts(CAMPAIGN_ID, PostInclude::empty())
            .await
            .unwrap();

        client
            .campaign_posts(CAMPAIGN_ID, PostInclude::all())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_post() {
        const POST_ID: u64 = 40846530;

        let client = PatreonClientBuilder::new(API_KEY, CLIENT_ID)
            .build()
            .unwrap();

        client.post(POST_ID, PostInclude::empty()).await.unwrap();

        client.post(POST_ID, PostInclude::all()).await.unwrap();
    }
}
