use tokio::join;

use crate::bot::discord::DiscordBot;
use crate::core::db::Database;
use crate::core::service::Service;
use crate::core::test;

use dotenvy::dotenv;

use std::env;

use std::sync::Arc;
use std::sync::Mutex;

mod bot;
mod core;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting!");
    // Test our services
    test();
    // Database
    // TODO: Fix mess of a line
    let mut db = Database::new(
        env::current_dir()
            .expect("Could not get current directory")
            .join("database")
            .to_str()
            .unwrap()
            .to_owned(),
    );
    db.start();
    db.ensure_collection("hourly")
        .expect("Database collection add fail 1. ");
    db.ensure_collection("daily")
        .expect("Database collection add fail 2. ");
    db.ensure_collection("weekly")
        .expect("Database collection add fail 3. ");
    db.save_meta().expect("Save fail");

    let db_arc = Arc::new(Mutex::new(db));

    // Create our services
    let mut bot = DiscordBot::new(
        db_arc,
        env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set"),
    )
    .await;
    // Lmao so we just uh init all here
    let bot_fut = bot.start();
    join!(bot_fut);

    println!("Init Done!");
}
