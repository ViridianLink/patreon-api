use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PledgeEvent {
    amount_cents: u64,
    currency_code: String,
    date: DateTime<Utc>,
    pledge_payment_status: Option<String>,
    payment_status: Option<String>,
    tier_id: String,
    tier_title: String,
    #[serde(rename = "type")]
    kind: String,
}
