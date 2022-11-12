use super::feed::{FeedJob,FeedItem};

pub struct FeedUpdatedMessage {
    pub job: FeedJob,
    pub item: FeedItem,
}

pub enum ServiceMessage {
    FeedUpdated(FeedUpdatedMessage),
}