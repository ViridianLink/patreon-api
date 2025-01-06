use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Benefit {
    app_external_id: Option<String>,
    app_meta: Option<serde_json::Value>,
    benefit_type: String,
    created_at: DateTime<Utc>,
    deliverables_due_today_count: u64,
    delivered_deliverables_count: u64,
    description: Option<String>,
    is_deleted: bool,
    is_ended: bool,
    is_published: bool,
    next_deliverable_due_date: Option<DateTime<Utc>>,
    not_delivered_deliverables_count: u64,
    rule_type: Option<String>,
    tiers_count: u64,
    title: String,
}

impl Benefit {
    pub(crate) fn as_query<'a>() -> &'a str {
        "app_external_id,app_meta,benefit_type,created_at,deliverables_due_today_count,delivered_deliverables_count,description,is_deleted,is_ended,is_published,next_deliverable_due_date,not_delivered_deliverables_count,rule_type,tiers_count,title"
    }
}
