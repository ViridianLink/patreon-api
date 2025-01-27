use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Post {
    pub app_id: Option<i64>,
    pub app_status: Option<String>,
    pub content: String,
    pub embed_data: Option<serde_json::Value>,
    pub embed_url: Option<String>,
    pub is_paid: bool,
    pub is_public: bool,
    pub tiers: Vec<serde_json::Value>,
    pub published_at: DateTime<Utc>,
    pub title: String,
    pub url: String,
}

impl Post {
    pub(crate) fn as_query<'a>() -> &'a str {
        "app_id,app_status,content,embed_data,embed_url,is_paid,is_public,tiers,published_at,title,url"
    }
}
