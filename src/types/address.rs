use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    pub addressee: String,
    pub city: String,
    pub country: String,
    pub created_at: DateTime<Utc>,
    pub line_1: String,
    pub line_2: String,
    pub phone_number: String,
    pub postal_code: String,
    pub state: String,
}

impl Address {
    pub(crate) fn as_query<'a>() -> &'a str {
        "addressee,city,country,created_at,line_1,line_2,phone_number,postal_code,state"
    }
}
