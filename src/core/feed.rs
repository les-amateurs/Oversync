use http::Uri;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeedItem {
    title: String,
    description: String,
    link: Option<String>,
    author: Option<String>,
    comments: Option<String>,
    origin: String, // currently just "rss"
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
    dest_type: String,
    id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeedJob{
    #[serde(with = "http_serde::uri")]
    uri: Uri,
    last_hash: Option<u64>,
    feed_type: String, // rss for now
    destination: FeedDestination
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeedCollection {
    jobs: Vec<FeedJob>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeedConfig{
    hourly: Option<FeedCollection>,
    daily: Option<FeedCollection>,
    weekly: Option<FeedCollection>,
}