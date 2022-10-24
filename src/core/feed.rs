use http::Uri;

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

pub struct FeedDestination {
    dest_type: String,
    id: String,
}

pub struct FeedJob{
    uri: Uri,
    last_hash: Option<u64>,
    feed_type: String,
    destination: FeedDestination
}

pub struct FeedCollection {
    jobs: Vec<FeedJob>,
}

pub struct FeedConfig{
    hourly: Option<FeedCollection>,
    daily: Option<FeedCollection>,
    weekly: Option<FeedCollection>,
}