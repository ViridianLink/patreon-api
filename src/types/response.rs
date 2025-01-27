use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Address, Benefit, Campaign, Goal, Member, PledgeEvent, Post, Tier, User};

pub type IdentityResponse = PatreonResponse<ResourceData<User>, IdentityIncluded>;
pub type CampaignsResponse = PatreonResponse<Vec<ResourceData<Campaign>>, CampaignsIncluded>;
pub type CampaignResponse = PatreonResponse<ResourceData<Campaign>, CampaignsIncluded>;
pub type CampaignMembersResponse = PatreonResponse<Vec<ResourceData<Member>>, MemberIncluded>;
pub type MemberResponse = PatreonResponse<ResourceData<Member>, MemberIncluded>;
pub type CampaignPostsResponse = PatreonResponse<Vec<ResourceData<Post>>, PostIncluded>;
pub type PostResponse = PatreonResponse<ResourceData<Post>, PostIncluded>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PatreonResponse<D, I> {
    pub data: D,
    #[serde(default = "Vec::new")]
    pub included: Vec<I>,
    pub links: Option<Links>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pagination {
    pub cursors: Cursors,
    pub total: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cursors {
    pub next: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceData<D> {
    pub attributes: D,
    pub id: String,
    #[serde(default)]
    pub relationships: HashMap<String, Value>,
    #[serde(rename = "type")]
    resource_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum IdentityIncluded {
    Campaign(ResourceData<Campaign>),
    Member(ResourceData<Member>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum CampaignsIncluded {
    Tier(ResourceData<Tier>),
    User(ResourceData<User>),
    Benefit(ResourceData<Benefit>),
    Goal(ResourceData<Goal>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum PostIncluded {
    Campaign(ResourceData<Campaign>),
    User(ResourceData<User>),
}
