use std::env;

use helferbiene_rs::handler::Handler;

use serenity::prelude::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("helferbiene_rs=info"));

    let token = env::var("BOT_TOKEN").expect("Please provide a valid bot token in ENV:BOT_TOKEN!");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}
