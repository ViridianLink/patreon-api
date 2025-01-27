use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Goal {
    pub amount_cents: u64,
    pub completed_percentage: u64,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub reached_at: DateTime<Utc>,
    pub title: String,
}

impl Goal {
    pub(crate) fn as_query<'a>() -> &'a str {
        "amount_cents,completed_percentage,created_at,description,reached_at,title"
    }
}
