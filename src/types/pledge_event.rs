use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PledgeEvent {
    pub amount_cents: u64,
    pub currency_code: String,
    pub date: DateTime<Utc>,
    pub pledge_payment_status: Option<String>,
    pub payment_status: Option<String>,
    pub tier_id: String,
    pub tier_title: String,
    #[serde(rename = "type")]
    pub kind: String,
}
