use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

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
        // user agent format: oversync/0.2.0 (version subject to change)
        let client = Client::builder().user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        )).build().unwrap_or_else(|_| Client::new());

        FetcherContext {
            client
        }
    }
}

struct UnseenFeeds {
    unseen_feeds: Vec<FeedItem>,
    new_hash: u64
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

    fn item_hash(item: &FeedItem) -> u64{
        let mut dh = DefaultHasher::new();
        item.hash(&mut dh);
        dh.finish()
    }

    fn get_new_feeds(feeds: &Vec<FeedItem>, last_seen_hash: Option<u64>) -> UnseenFeeds {
        let new_last_hash: u64 = 0;
        if !feeds.is_empty() {
            // i'm sure this won't panic since I ensure it has at least one
            let new_last_hash = Self::item_hash(feeds.get(0).unwrap());
            let mut unseen_feeds: Vec<FeedItem> = vec![];
            let last_seen_hash = last_seen_hash.unwrap_or(0);
            for feed in feeds {
                // Calculate hash of current feed
                let current_hash = Self::item_hash(feed);
                if current_hash == last_seen_hash {
                    break;
                }
                unseen_feeds.push(feed.clone()); // TODO: avoid nasty copy
            }
            UnseenFeeds {
                unseen_feeds,
                new_hash: new_last_hash
            }
        }else{
            UnseenFeeds {
                unseen_feeds: feeds.to_vec(), // copy 
                new_hash: new_last_hash
            }
        }
    }
}