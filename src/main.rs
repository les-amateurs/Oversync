use tokio::join;

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
    
    // Create our services
    let mut bot = DiscordBot::new(env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set")).await;
    // Lmao so we just uh init all here
    let bot_fut = bot.start();
    join!(bot_fut);

    println!("Done!");

}
