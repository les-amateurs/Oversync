use http::Uri;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeedItem {
    pub title: String,
    pub description: String,
    pub link: Option<String>,
    pub author: Option<String>,
    pub comments: Option<String>,
    pub origin: String, // currently just "rss"
}

impl Default for FeedItem {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            description: "Description not provided".to_string(),
            link: None,
            author: None,
            comments: None,
            origin: "unknown".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeedDestination {
    pub dest_type: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeedJob{
    #[serde(with = "http_serde::uri")]
    pub uri: Uri,
    pub last_hash: Option<u64>,
    pub feed_type: String, // rss for now
    pub destination: FeedDestination
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeedCollection {
    pub jobs: Vec<FeedJob>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeedConfig{
    pub hourly: Option<FeedCollection>,
    pub daily: Option<FeedCollection>,
    pub weekly: Option<FeedCollection>,
}