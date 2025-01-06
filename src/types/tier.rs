use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tier {
    amount_cents: u64,
    created_at: DateTime<Utc>,
    description: String,
    discord_role_ids: Option<Vec<String>>,
    edited_at: DateTime<Utc>,
    image_url: Option<String>,
    patron_count: u64,
    post_count: u64,
    published: bool,
    published_at: Option<DateTime<Utc>>,
    remaining: Option<u64>,
    requires_shipping: bool,
    title: String,
    unpublished_at: Option<DateTime<Utc>>,
    url: String,
    user_limit: Option<u64>,
}

impl Tier {
    pub(crate) fn as_query<'a>() -> &'a str {
        "amount_cents,created_at,description,discord_role_ids,edited_at,image_url,patron_count,post_count,published,published_at,remaining,requires_shipping,title,unpublished_at,url,user_limit"
    }
}
