use tokio::join;

use crate::core::db::Database;
use crate::core::service::Service;
use crate::core::test;
use crate::bot::discord::DiscordBot;

use dotenvy::dotenv;

use std::env;

mod core;
mod bot;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting!");
    // Test our services    
    test();
    // Database
    // TODO: Fix mess of a line
    let mut db = Database::new(env::current_dir().expect("Could not get current directory").join("database").to_str().unwrap().to_owned());
    db.start();
    db.ensure_collection("hourly");
    db.ensure_collection("daily");
    db.ensure_collection("weekly");
    db.save_meta().expect("Save fail");

    // Create our services
    let mut bot = DiscordBot::new(env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set")).await;
    // Lmao so we just uh init all here
    let bot_fut = bot.start();
    join!(bot_fut);

    println!("Done!");

}
