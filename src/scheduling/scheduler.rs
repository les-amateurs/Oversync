use std::io::Error;
use std::sync::{Mutex, Arc};

// Scheduler, and trait for .seconds(), .minutes(), etc.
// Import week days and WeekDay
use clokwerk::{TimeUnits};
use clokwerk::Interval::*;
use std::thread;
use std::time::Duration;

use async_trait::async_trait;

use crate::core::feed::FeedCollection;
use crate::{core::{service::Service, messaging::{ServiceMessage, FeedUpdatedMessage}, db::Database, feed::{FeedItem, FeedJob}}, bot::discord::DiscordBot};

// For now the scheduler owns the other services. 
pub struct Scheduler{
    pub db_arc: Arc<Mutex<Database>>,
    pub bot: Option<DiscordBot>,
    pub cw_scheduler: Arc<Mutex<clokwerk::Scheduler>>,
}

impl Scheduler {
    fn signal_update(&mut self, item: FeedItem,job: FeedJob){
        let message = ServiceMessage::FeedUpdated(FeedUpdatedMessage {
            item,
            job,
        });
        let message_arc = Arc::new(&message);
        if let Some(bot) = &self.bot {
            bot.recieve(message_arc.clone());
        }
    }

    pub fn set_bot(&mut self, bot: DiscordBot){
        self.bot = Some(bot);
    }

    async fn try_update(&mut self, job: &FeedJob) -> std::io::Result<()>{
        // delegate to feed fetcher that determines which fetcher to use
    }

    async fn try_update_collection(&mut self, collection_name: &str, required_time: chrono::Duration) -> std::io::Result<()> {
        self.db_arc.lock().unwrap().iterate_collection::<FeedCollection>(collection_name)?.for_each(|result| {
            if let Ok(feed_collection) = result {
                for job in feed_collection.jobs  {
                    let time_since = chrono::Utc::now() - job.last_synced;
                    if time_since >= required_time {
                        // Sync now
                        self.try_update(&job).await?;
                    }
                }
            }
        });
        Ok(())
    }

    pub fn new(db_arc: Arc<Mutex<Database>>) -> Self {
        Self {
            db_arc,
            bot: None,
            cw_scheduler: Arc::new(Mutex::new(clokwerk::Scheduler::new())),
        }
    }
}

#[async_trait]
impl Service for Scheduler{
    async fn recieve(&self, message: Arc<&ServiceMessage>) {
        match message.as_ref() {
            ServiceMessage::FeedUpdated(_) => todo!(),
        }
    }
    
    async fn start(&mut self) {
        let mut cw_scheduler = self.cw_scheduler.lock().unwrap();
        cw_scheduler.every(60.minutes()).run(|| println!("Test"));
        println!("Registered scheduled events");
        let cw_scheduler_clone = self.cw_scheduler.clone();
        tokio::spawn(async move {
            loop {
              cw_scheduler_clone.lock().unwrap().run_pending();
              tokio::time::sleep(Duration::from_secs(60)).await;
            }
        });
    }

}