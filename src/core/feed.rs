use http::Uri;

struct FeedItem {
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

struct FeedDestination {
    dest_type: String,
    id: String,
}

struct FeedJob{
    uri: Uri,
    last_hash: Option<u64>,
    feed_type: String,
    destination: FeedDestination
}

struct FeedCollection {
    jobs: Vec<FeedJob>,
}

struct FeedConfig{
    hourly: Option<FeedCollection>,
    daily: Option<FeedCollection>,
    weekly: Option<FeedCollection>,
}