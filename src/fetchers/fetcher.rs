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

pub struct UnseenItems {
    pub unseen_items: Vec<FeedItem>,
    pub new_hash: u64
}

#[async_trait]
pub trait Fetcher{
    async fn fetch(context: &mut FetcherContext, job: &FeedJob) -> anyhow::Result<Vec<FeedItem>> {
        Ok(vec!()) // unknown type, we return empty item for now
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

    fn get_new_items(items: &Vec<FeedItem>, last_seen_hash: Option<u64>) -> UnseenItems {
        let new_last_hash: u64 = 0;
        if !items.is_empty() {
            // i'm sure this won't panic since I ensure it has at least one
            let new_last_hash = Self::item_hash(items.get(0).unwrap());
            let mut unseen_items: Vec<FeedItem> = vec![];
            let last_seen_hash = last_seen_hash.unwrap_or(0);
            for item in items {
                // Calculate hash of current item
                let current_hash = Self::item_hash(item);
                if current_hash == last_seen_hash {
                    break;
                }
                unseen_items.push(item.clone()); // TODO: avoid nasty copy
            }
            UnseenItems {
                unseen_items,
                new_hash: new_last_hash
            }
        }else{
            UnseenItems {
                unseen_items: items.to_vec(), // copy 
                new_hash: new_last_hash
            }
        }
    }
}

pub struct DefaultFetcher {

}

impl Fetcher for DefaultFetcher {

}