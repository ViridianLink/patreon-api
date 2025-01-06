use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Benefit {
    pub app_external_id: Option<String>,
    pub app_meta: Option<serde_json::Value>,
    pub benefit_type: String,
    pub created_at: DateTime<Utc>,
    pub deliverables_due_today_count: u64,
    pub delivered_deliverables_count: u64,
    pub description: Option<String>,
    pub is_deleted: bool,
    pub is_ended: bool,
    pub is_published: bool,
    pub next_deliverable_due_date: Option<DateTime<Utc>>,
    pub not_delivered_deliverables_count: u64,
    pub rule_type: Option<String>,
    pub tiers_count: u64,
    pub title: String,
}

impl Benefit {
    pub(crate) fn as_query<'a>() -> &'a str {
        "app_external_id,app_meta,benefit_type,created_at,deliverables_due_today_count,delivered_deliverables_count,description,is_deleted,is_ended,is_published,next_deliverable_due_date,not_delivered_deliverables_count,rule_type,tiers_count,title"
    }
}
