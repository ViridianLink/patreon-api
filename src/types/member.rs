use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Member {
    pub campaign_lifetime_support_cents: i32,
    pub currently_entitled_amount_cents: i32,
    pub email: Option<String>,
    pub full_name: String,
    pub is_follower: bool,
    pub is_free_trial: bool,
    pub is_gifted: bool,
    pub last_charge_date: Option<DateTime<Utc>>,
    pub last_charge_status: Option<String>,
    pub lifetime_support_cents: i32,
    pub next_charge_date: Option<DateTime<Utc>>,
    pub note: String,
    pub patron_status: Option<String>,
    pub pledge_cadence: Option<i32>,
    pub pledge_relationship_start: DateTime<Utc>,
    pub will_pay_amount_cents: i32,
}

impl Member {
    pub(crate) fn as_query<'a>() -> &'a str {
        "campaign_lifetime_support_cents,currently_entitled_amount_cents,email,full_name,is_follower,is_free_trial,is_gifted,last_charge_date,last_charge_status,lifetime_support_cents,next_charge_date,note,patron_status,pledge_cadence,pledge_relationship_start,will_pay_amount_cents"
    }
}
