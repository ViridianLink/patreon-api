use serde::{Deserialize, Serialize};

use super::{Address, Benefit, Campaign, Goal, Member, PledgeEvent, Post, Tier, User};

pub type IdentityResponse = PatreonResponse<ResourceData<User>, IdentityIncluded>;
pub type CampaignsResponse = PatreonResponse<Vec<ResourceData<Campaign>>, CampaignsIncluded>;
pub type CampaignResponse = PatreonResponse<ResourceData<Campaign>, CampaignsIncluded>;
pub type CampaignMembersResponse = PatreonResponse<Vec<ResourceData<Member>>, MemberIncluded>;
pub type MemberResponse = PatreonResponse<ResourceData<Member>, MemberIncluded>;
pub type CampaignPostsResponse = PatreonResponse<Vec<ResourceData<Post>>, PostIncluded>;

#[derive(Debug, Deserialize, Serialize)]
pub struct PatreonResponse<D, I> {
    data: D,
    #[serde(default = "Vec::new")]
    included: Vec<I>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceData<D> {
    attributes: D,
    id: String,
    // #[serde(rename = "type")]
    // resource_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum IdentityIncluded {
    Campaign(ResourceData<Campaign>),
    Member(ResourceData<Member>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum CampaignsIncluded {
    Tier(ResourceData<Tier>),
    User(ResourceData<User>),
    Benefit(ResourceData<Benefit>),
    Goal(ResourceData<Goal>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum MemberIncluded {
    Address(ResourceData<Address>),
    Campaign(ResourceData<Campaign>),
    Tier(ResourceData<Tier>),
    User(ResourceData<User>),
    PledgeEvent(ResourceData<PledgeEvent>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum PostIncluded {
    Campaign(ResourceData<Campaign>),
    User(ResourceData<User>),
}
