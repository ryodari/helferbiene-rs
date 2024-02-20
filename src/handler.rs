use crate::commands::COMMANDS;
use serenity::all::Color;
use serenity::all::Command;
use serenity::all::CreateEmbed;
use serenity::all::CreateInteractionResponse;
use serenity::all::CreateInteractionResponseMessage;

use serenity::all::EditInteractionResponse;
use serenity::all::Interaction;
use serenity::all::Ready;
use serenity::async_trait;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            log::debug!("Received command interaction: {command:#?}");

            let command_name = command.data.name.as_str();

            let handler = match COMMANDS.iter().find(|h| h.name() == command_name) {
                Some(h) => h,
                None => {
                    log::warn!("Couldn't find handler for command: \"{}\"", command_name);

                    let embed = CreateEmbed::new()
                        .title("Error")
                        .description("Invalid command.")
                        .color(Color::from_rgb(255, 0, 0));

                    _ = command
                        .create_response(
                            ctx.http(),
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().add_embed(embed),
                            ),
                        )
                        .await;

                    return;
                }
            };

            match handler.run(&ctx, &command).await {
                Ok(Some(resp)) => {
                    if let Err(e) = command.create_response(&ctx.http, resp).await {
                        log::error!("Couldn't respond to slash command: {}", e);
                    }
                }
                Err(e) => {
                    log::error!("Executing command \"{}\" failed: {}", command_name, e);

                    let embed = CreateEmbed::new()
                        .title("Error")
                        .description("An error occured while executing the command.")
                        .color(Color::from_rgb(255, 0, 0));

                    let res = match command.get_response(&ctx.http).await.is_ok() {
                        true => command
                            .edit_response(
                                ctx.http(),
                                EditInteractionResponse::new().add_embed(embed),
                            )
                            .await
                            .map(|_| ()),
                        false => {
                            command
                                .create_response(
                                    ctx.http(),
                                    CreateInteractionResponse::Message(
                                        CreateInteractionResponseMessage::new().add_embed(embed),
                                    ),
                                )
                                .await
                        }
                    };

                    if let Err(e) = res {
                        log::error!("Couldn't respond to slash command: {}", e);
                    }
                }
                _ => {}
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);

        for (idx, cmd) in COMMANDS.iter().enumerate() {
            match Command::create_global_command(&ctx.http, cmd.register()).await {
                Ok(_) => log::info!(
                    "Registered command \"{}\" ({}/{})",
                    cmd.name(),
                    idx + 1,
                    COMMANDS.len()
                ),
                Err(e) => log::error!(
                    "Failed to register command \"{}\" ({}/{}): {}",
                    cmd.name(),
                    idx + 1,
                    COMMANDS.len(),
                    e
                ),
            };
        }
    }
}
