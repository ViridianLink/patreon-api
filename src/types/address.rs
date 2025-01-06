use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    addressee: String,
    city: String,
    country: String,
    created_at: DateTime<Utc>,
    line_1: String,
    line_2: String,
    phone_number: String,
    postal_code: String,
    state: String,
}

impl Address {
    pub(crate) fn as_query<'a>() -> &'a str {
        "addressee,city,country,created_at,line_1,line_2,phone_number,postal_code,state"
    }
}
