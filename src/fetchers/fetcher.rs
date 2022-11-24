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
        let client_builder = Client::builder();
        client_builder.user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        )); // So Oversync/0.2.0 (this may change)
        
        FetcherContext {
            client: client_builder.build()
        }
    }
}
#[async_trait]
pub trait Fetcher{
    async fn fetch(context: &mut FetcherContext, job: &FeedJob) -> anyhow::Result<Vec<FeedItem>> {
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