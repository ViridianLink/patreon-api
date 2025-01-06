use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Goal {
    amount_cents: u64,
    completed_percentage: u64,
    created_at: DateTime<Utc>,
    description: String,
    reached_at: DateTime<Utc>,
    title: String,
}

impl Goal {
    pub(crate) fn as_query<'a>() -> &'a str {
        "amount_cents,completed_percentage,created_at,description,reached_at,title"
    }
}
