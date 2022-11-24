use async_trait::async_trait;
use http::Uri;
use rss::Channel;

use crate::core::feed::{FeedJob,FeedItem};

use super::fetcher::{Fetcher, FetcherContext};

pub struct RSSFetcher {

}

#[async_trait]
impl Fetcher for RSSFetcher {
    async fn fetch(context: &mut FetcherContext, job: &FeedJob) -> anyhow::Result<Vec<FeedItem>> {
        let req_builder = Self::get(context, &job.uri);
        let response = req_builder.send().await?;
        
        let response_bytes = response.bytes().await?;
        
        let channel = Channel::read_from(&response_bytes[..])?;

        // TODO: avoid excessive copying
        Ok(channel.items.iter().map(|item| FeedItem {
            author: item.author.clone(),
            link: item.link.clone(),
            title: item.title.clone().unwrap_or_else(|| "Untitled RSS item. ".to_owned()),
            description: item.description.clone().unwrap_or_else(|| "RSS item had no desc. ".to_owned()),
            comments: None, // new comments will alter hash, TODO: impl custom hash
            origin: "rss".to_owned()
        }).collect())

    }
}