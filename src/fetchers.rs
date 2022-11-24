use crate::core::feed::{FeedItem, FeedJob};

use self::fetcher::{FetcherContext, Fetcher};

pub mod fetcher;
pub mod rss;
// pub mod something

pub fn fetch_any(context: &FetcherContext, job: &FeedJob) -> std::io::Result<Vec<FeedItem>> {
    match job.feed_type.as_ref() {
        "rss" => self::rss::RSSFetcher::fetch(&mut context, job),
        _ => Ok(vec!()) // unknown type, we return empty feed for now
    }
}