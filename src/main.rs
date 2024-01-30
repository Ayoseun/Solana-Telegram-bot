
use teloxide::prelude::*;
use std::env;
use std::error::Error;
mod api;
mod bot;
use api::live;
use crate::bot::{cmd,telegramBot};

#[macro_use]
extern crate rocket;

async fn start() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
      // Retrieve the API token from environment variables
      let api_token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN not found in environment variables");

      // Create a Bot instance using the retrieved API token
      let bot = Bot::new(api_token);

    cmd::repl(bot, telegramBot).await;
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv();
    tokio::spawn(start());
    rocket::build()
        .mount("/", routes![live])
        .launch()
        .await
        .unwrap();
}
