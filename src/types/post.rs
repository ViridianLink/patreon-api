use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    app_id: Option<i64>,
    app_status: Option<String>,
    content: String,
    embed_data: Option<serde_json::Value>,
    embed_url: Option<String>,
    is_paid: bool,
    is_public: bool,
    tiers: Vec<serde_json::Value>,
    published_at: DateTime<Utc>,
    title: String,
    url: String,
}

impl Post {
    pub(crate) fn as_query<'a>() -> &'a str {
        "app_id,app_status,content,embed_data,embed_url,is_paid,is_public,tiers,published_at,title,url"
    }
}
