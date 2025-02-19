use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub about: Option<String>,
    pub can_see_nsfw: Option<bool>,
    pub created: DateTime<Utc>,
    pub email: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub full_name: String,
    pub hide_pledges: bool,
    pub image_url: String,
    pub is_email_verified: Option<bool>,
    pub like_count: u32,
    #[serde(default)]
    pub social_connections: HashMap<String, Option<SocialConnection>>,
    pub thumb_url: String,
    pub url: String,
    pub vanity: Option<String>,
    pub is_creator: bool,
}

impl User {
    pub(crate) fn as_query<'a>() -> &'a str {
        "about,can_see_nsfw,created,email,first_name,full_name,hide_pledges,image_url,is_email_verified,like_count,social_connections,thumb_url,url,vanity,is_creator"
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SocialConnection {
    pub scopes: Option<Vec<String>>,
    pub url: Option<String>,
    pub user_id: Option<String>,
}
