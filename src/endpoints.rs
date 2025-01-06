use url::Url;

use crate::patreon_client::PatreonClient;
use crate::types::includes::{CampaignInclude, IdentityInclude, MemberInclude, PostInclude};
use crate::types::response::{
    CampaignMembersResponse, CampaignPostsResponse, CampaignResponse, CampaignsResponse,
    IdentityResponse, MemberResponse,
};
use crate::types::{Address, Benefit, Campaign, Goal, Member, Post, Tier, User};
use crate::Result;

const BASE_URI: &str = "https://www.patreon.com";

impl PatreonClient {
    pub async fn identity(&self, includes: IdentityInclude) -> Result<IdentityResponse> {
        let mut url = Url::parse(BASE_URI).unwrap();
        url.set_path("/api/oauth2/v2/identity");
        url.query_pairs_mut()
            .append_pair("fields[user]", User::as_query());

        if !includes.is_empty() {
            url.query_pairs_mut()
                .append_pair("include", &includes.as_query());
        }
        if includes.contains(IdentityInclude::MEMBERSHIPS) {
            url.query_pairs_mut()
                .append_pair("fields[member]", Member::as_query());
        }
        if includes.contains(IdentityInclude::CAMPAIGN) {
            url.query_pairs_mut()
                .append_pair("fields[campaign]", Campaign::as_query());
        }

        self.get(url).await
    }

    fn campaign_query(url: &mut Url, includes: CampaignInclude) {
        url.query_pairs_mut()
            .append_pair("fields[campaign]", Campaign::as_query());

        if !includes.is_empty() {
            url.query_pairs_mut()
                .append_pair("include", &includes.as_query());
        }
        if includes.contains(CampaignInclude::TIERS) {
            url.query_pairs_mut()
                .append_pair("fields[tier]", Tier::as_query());
        }
        if includes.contains(CampaignInclude::CREATOR) {
            url.query_pairs_mut()
                .append_pair("fields[user]", User::as_query());
        }
        if includes.contains(CampaignInclude::BENEFITS) {
            url.query_pairs_mut()
                .append_pair("fields[benefit]", Benefit::as_query());
        }
        if includes.contains(CampaignInclude::GOALS) {
            url.query_pairs_mut()
                .append_pair("fields[goal]", Goal::as_query());
        }
    }

    pub async fn campaigns(&self, includes: CampaignInclude) -> Result<CampaignsResponse> {
        let mut url = Url::parse(BASE_URI).unwrap();
        url.set_path("/api/oauth2/v2/campaigns");

        Self::campaign_query(&mut url, includes);

        self.get(url).await
    }

    pub async fn campaign(
        &self,
        campaign_id: u64,
        includes: CampaignInclude,
    ) -> Result<CampaignResponse> {
        let mut url = Url::parse(BASE_URI).unwrap();
        url.set_path(&format!("/api/oauth2/v2/campaigns/{}", campaign_id));

        Self::campaign_query(&mut url, includes);

        self.get(url).await
    }

    pub async fn campaign_members(
        &self,
        campaign_id: u64,
        includes: MemberInclude,
    ) -> Result<CampaignMembersResponse> {
        let mut url = Url::parse(BASE_URI).unwrap();
        url.set_path(&format!("/api/oauth2/v2/campaigns/{}/members", campaign_id));

        url.query_pairs_mut()
            .append_pair("fields[member]", Member::as_query());

        if !includes.is_empty() {
            url.query_pairs_mut()
                .append_pair("include", &includes.as_query());
        }
        if includes.contains(MemberInclude::ADDRESS) {
            url.query_pairs_mut()
                .append_pair("fields[address]", Address::as_query());
        }
        if includes.contains(MemberInclude::CAMPAIGN) {
            url.query_pairs_mut()
                .append_pair("fields[campaign]", Campaign::as_query());
        }
        if includes.contains(MemberInclude::CURRENTLY_ENTITLED_TIERS) {
            url.query_pairs_mut()
                .append_pair("fields[tier]", Tier::as_query());
        }
        if includes.contains(MemberInclude::USER) {
            url.query_pairs_mut()
                .append_pair("fields[user]", User::as_query());
        }

        self.get(url).await
    }

    pub async fn member(&self, member_id: &str, includes: MemberInclude) -> Result<MemberResponse> {
        let mut url = Url::parse(BASE_URI).unwrap();
        url.set_path(&format!("/api/oauth2/v2/members/{}", member_id));

        url.query_pairs_mut()
            .append_pair("fields[member]", Member::as_query());

        if !includes.is_empty() {
            url.query_pairs_mut()
                .append_pair("include", &includes.as_query());
        }
        if includes.contains(MemberInclude::ADDRESS) {
            url.query_pairs_mut()
                .append_pair("fields[address]", Address::as_query());
        }
        if includes.contains(MemberInclude::CAMPAIGN) {
            url.query_pairs_mut()
                .append_pair("fields[campaign]", Campaign::as_query());
        }
        if includes.contains(MemberInclude::CURRENTLY_ENTITLED_TIERS) {
            url.query_pairs_mut()
                .append_pair("fields[tier]", Tier::as_query());
        }
        if includes.contains(MemberInclude::USER) {
            url.query_pairs_mut()
                .append_pair("fields[user]", User::as_query());
        }

        self.get(url).await
    }

    fn post_query(url: &mut Url, includes: PostInclude) {
        url.query_pairs_mut()
            .append_pair("fields[post]", Post::as_query());

        if !includes.is_empty() {
            url.query_pairs_mut()
                .append_pair("include", &includes.as_query());
        }
        if includes.contains(PostInclude::USER) {
            url.query_pairs_mut()
                .append_pair("fields[user]", User::as_query());
        }
        if includes.contains(PostInclude::CAMPAIGN) {
            url.query_pairs_mut()
                .append_pair("fields[campaign]", Campaign::as_query());
        }
    }

    pub async fn campaign_posts(
        &self,
        campaign_id: u64,
        includes: PostInclude,
    ) -> Result<CampaignPostsResponse> {
        let mut url = Url::parse(BASE_URI).unwrap();
        url.set_path(&format!("/api/oauth2/v2/campaigns/{}/posts", campaign_id));

        Self::post_query(&mut url, includes);

        self.get(url).await
    }

    pub async fn post(&self, post_id: u64, includes: PostInclude) -> Result<serde_json::Value> {
        let mut url = Url::parse(BASE_URI).unwrap();
        url.set_path(&format!("/api/oauth2/v2/posts/{}", post_id));

        Self::post_query(&mut url, includes);

        self.get(url).await
    }
}
