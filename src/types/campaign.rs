use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Campaign {
    pub created_at: DateTime<Utc>,
    pub creation_name: String,
    pub discord_server_id: String,
    pub google_analytics_id: String,
    pub has_rss: bool,
    pub has_sent_rss_notify: bool,
    pub image_small_url: String,
    pub image_url: String,
    pub is_charged_immediately: bool,
    pub is_monthly: bool,
    pub is_nsfw: bool,
    pub main_video_embed: String,
    pub main_video_url: String,
    pub one_liner: Option<String>,
    pub patron_count: i32,
    pub pay_per_name: String,
    pub pledge_url: String,
    pub published_at: DateTime<Utc>,
    pub rss_artwork_url: Option<String>,
    pub rss_feed_title: Option<String>,
    pub show_earnings: Option<bool>,
    pub summary: String,
    pub thanks_embed: Option<String>,
    pub thanks_msg: Option<String>,
    pub thanks_video_url: Option<String>,
    pub url: String,
    pub vanity: String,
}

impl Campaign {
    pub(crate) fn as_query<'a>() -> &'a str {
        "created_at,creation_name,discord_server_id,google_analytics_id,has_rss,has_sent_rss_notify,image_small_url,image_url,is_charged_immediately,is_monthly,is_nsfw,main_video_embed,main_video_url,one_liner,patron_count,pay_per_name,pledge_url,published_at,rss_artwork_url,rss_feed_title,show_earnings,summary,thanks_embed,thanks_msg,thanks_video_url,url,vanity"
    }
}
