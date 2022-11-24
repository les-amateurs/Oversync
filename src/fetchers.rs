use crate::core::feed::{FeedItem, FeedJob};

use self::fetcher::{FetcherContext, Fetcher};

pub mod fetcher;
pub mod rss;
// pub mod something

pub async fn fetch_any(context: &mut FetcherContext, job: &FeedJob) -> std::result::Result<Vec<FeedItem>> {
    match job.feed_type.as_ref() {
        "rss" => Ok(self::rss::RSSFetcher::fetch(context, job).await?),
        _ => Ok(vec!()) // unknown type, we return empty feed for now
    }
}