use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tier {
    pub amount_cents: u64,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub discord_role_ids: Option<Vec<String>>,
    pub edited_at: DateTime<Utc>,
    pub image_url: Option<String>,
    pub patron_count: u64,
    pub post_count: u64,
    pub published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub remaining: Option<u64>,
    pub requires_shipping: bool,
    pub title: String,
    pub unpublished_at: Option<DateTime<Utc>>,
    pub url: String,
    pub user_limit: Option<u64>,
}

impl Tier {
    pub(crate) fn as_query<'a>() -> &'a str {
        "amount_cents,created_at,description,discord_role_ids,edited_at,image_url,patron_count,post_count,published,published_at,remaining,requires_shipping,title,unpublished_at,url,user_limit"
    }
}
