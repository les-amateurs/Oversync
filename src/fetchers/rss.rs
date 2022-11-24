use async_trait::async_trait;
use http::Uri;

use crate::core::feed::{FeedJob,FeedItem};

use super::fetcher::{Fetcher, FetcherContext};

pub struct RSSFetcher {

}

#[async_trait]
impl Fetcher for RSSFetcher {
    async fn fetch(context: &mut FetcherContext, job: &FeedJob) -> std::io::Result<Vec<FeedItem>> {
        let req_builder = Self::get(context, &job.uri);
        let response = req_builder.send().await?;
        
        Ok(vec![])
    }
}