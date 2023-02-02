use event_handler::Handler;
use serenity::{prelude::GatewayIntents, Client};

mod constants;
mod event_handler;

#[tokio::main]
pub async fn main() {

    // Get the token
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Init and handle the bot
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
