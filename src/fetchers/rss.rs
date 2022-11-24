use http::Uri;

use crate::core::feed::{FeedJob,FeedItem};

use super::fetcher::{Fetcher, FetcherContext};

pub struct RSSFetcher {

}

impl Fetcher for RSSFetcher {
    fn fetch(context: &mut FetcherContext, job: &FeedJob) -> std::io::Result<Vec<FeedItem>> {
        let req_builder = Self::get(context, &job.uri);
        req_builder.send();
    }
}