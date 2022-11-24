use crate::core::feed::{FeedJob,FeedItem};

use async_trait::async_trait;
use http::Uri;
use reqwest::{Client, Url, Response};

pub struct FetcherContext {
    // Reqwest Client
    pub client: Client
}

impl FetcherContext {
    pub fn new() -> Self{
        FetcherContext {
            client: Client::new()
        }
    }
}
#[async_trait]
pub trait Fetcher{
    async fn fetch(context: &mut FetcherContext, job: &FeedJob) -> std::result::Result<Vec<FeedItem>, std::error::Error> {
        Ok(vec!()) // unknown type, we return empty feed for now
    }

    fn get(context: &mut FetcherContext, uri: &Uri) -> reqwest::RequestBuilder {
        context.client.get(Self::as_url(uri))
    }

    fn as_url(uri: &Uri) -> Url{
        // this placeholder is kind of stupid
        Url::parse(&uri.to_string()).unwrap_or_else(|_| Url::parse("http://127.0.0.1").expect("Fallback hardcoded url doesn't parse properly?"))
    }
}