use std::env;

use helferbiene_rs::{handler::Handler, minecraft::activity::Activity};

use serenity::{all::OnlineStatus, prelude::*};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("helferbiene_rs=info"));

    let token = env::var("BOT_TOKEN").expect("Please provide a valid bot token in ENV:BOT_TOKEN!");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .status(OnlineStatus::Online)
        .await
        .expect("Error creating client");

    if let Ok(activity_server) = env::var("ACTIVITY_SERVER") {
        let split: Vec<&str> = activity_server.split(":").collect();

        let (host, port) = if split.len() >= 1 {
            (split[0].to_string(), split[1].parse::<u16>().unwrap())
        } else {
            (activity_server, 25565)
        };

        let shard_manager = client.shard_manager.clone();
        let activity_updater = Activity::new(host, port, shard_manager).await?;
        tokio::spawn(async {
            activity_updater.start().await;
        });
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start_autosharded().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}
