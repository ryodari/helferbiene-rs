use image::EncodableLayout;

use itertools::Itertools;
use serenity::{
    all::{
        Colour, CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand,
        CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
        CreateInteractionResponseMessage, EditInteractionResponse, ResolvedOption, ResolvedValue,
    },
    async_trait,
};

use crate::minecraft::{
    self,
    packet::slp::{SlpResponse, SlpServerDescription},
};

use super::Command;

pub struct ServerInfoCommand;

#[async_trait]
impl Command for ServerInfoCommand {
    fn name(&self) -> &'static str {
        "serverinfo"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Fetch a minecraft servers information")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "host",
                    "The servers IP/Hostname",
                )
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "port",
                    "The servers game port",
                )
                .min_int_value(u16::MIN as u64)
                .max_int_value(u16::MAX as u64)
                .required(false),
            )
    }

    async fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
    ) -> serenity::Result<Option<CreateInteractionResponse>> {
        let options = command.data.options();

        let (host, port) = match options.first() {
            Some(ResolvedOption {
                value: ResolvedValue::String(host),
                ..
            }) => {
                let split_host: Vec<&str> = host.split(":").collect();

                if split_host.len() >= 1 {
                    (split_host[0], split_host[0].parse::<u16>().ok())
                } else {
                    (*host, None)
                }
            }
            _ => {
                return Ok(Some(CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Please provide a host"),
                )))
            }
        };

        let port = match port {
            Some(port) => port,
            None => match options.iter().find(|o| o.name == "port") {
                Some(ResolvedOption {
                    value: ResolvedValue::Integer(port),
                    ..
                }) => *port as u16,
                _ => 25565,
            },
        };

        // defer
        command.defer(&ctx.http).await?;

        let client = minecraft::client::Client::new(host.to_string(), port).await?;
        let info: SlpResponse = client.status().await?;

        let description = match info.description {
            SlpServerDescription::Simple(ref description) => description.clone(),
            SlpServerDescription::Complex(ref component) => component.format_string(),
        };

        let favicon = info.favicon;

        let mut embed = CreateEmbed::new()
            .title(format!("{}:{}", host, port))
            .field("MOTD", description, false)
            .field(
                "Players",
                format!("{}/{}", info.players.online, info.players.max),
                false,
            )
            .footer(CreateEmbedFooter::new("helferbiene-rs"));

        if let Some(sample) = info.players.sample {
            if !sample.is_empty() {
                let all_players = sample.len() == info.players.online as usize;

                let mut formatted = sample
                    .iter()
                    .map(|p| format!("[{}](https://namemc.com/search?q={})", p.name, p.id))
                    .join("\n");

                if !all_players {
                    formatted.push_str("\n...");
                }

                embed = embed.field("", &formatted, false);
            }
        }

        embed = embed.field("Version", info.version.name, true).field(
            "Protocol",
            info.version.protocol.to_string(),
            true,
        );

        if let Some(mod_info) = info.modinfo {
            if mod_info.type_ == "FML" && !mod_info.mod_list.is_empty() {
                let formatted = mod_info
                    .mod_list
                    .iter()
                    .map(|m| format!("{}@{}", m.modid, m.version))
                    .join("\n");

                let chunks = split_into_chunks(formatted.as_str(), 1024);
                for (index, chunk) in chunks.iter().enumerate() {
                    let field_name = match index {
                        0 => "Mods",
                        _ => "",
                    };
                    embed = embed.field(field_name, chunk, false);
                }
            }
        }

        let mut response = EditInteractionResponse::new();

        if let Some(favicon) = favicon {
            let favicon = match favicon.strip_prefix("data:image/png;base64,") {
                Some(f) => f.to_string(),
                None => favicon,
            };

            if let Ok(decoded) = {
                use base64::Engine;
                base64::prelude::BASE64_STANDARD.decode(favicon)
            } {
                if let Ok(image) = image::load_from_memory(&decoded) {
                    match color_thief::get_palette(
                        image.to_rgba8().as_bytes(),
                        color_thief::ColorFormat::Rgba,
                        10,
                        2,
                    ) {
                        Ok(colors) => {
                            if let Some(color) = colors.first() {
                                embed = embed.colour(Colour::from_rgb(color.r, color.g, color.b));
                            }
                        }
                        Err(e) => log::error!("Failed to extract dominant color: {}", e),
                    };
                }

                let attachment = CreateAttachment::bytes(decoded, "favicon.png");
                response = response.new_attachment(attachment);
                embed = embed.thumbnail("attachment://favicon.png");
            }
        }

        response = response.add_embed(embed);

        command.edit_response(&ctx.http, response).await?;

        return Ok(None);
    }
}

fn split_into_chunks(input: &str, max_chunk_size: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_chunk = String::new();
    let mut current_size = 0;

    for line in input.lines() {
        // Check if adding the line exceeds the max_chunk_size
        if current_size + line.len() > max_chunk_size {
            // If it exceeds, push the current chunk into the result
            result.push(current_chunk.clone());
            // Reset current_chunk and current_size
            current_chunk.clear();
            current_size = 0;
        }

        // Add the line to the current_chunk
        current_chunk.push_str(line);
        current_chunk.push('\n'); // Add back the newline character
        current_size += line.len() + 1; // Add the length of the line plus 1 for the newline character
    }

    // Push the last chunk
    if !current_chunk.is_empty() {
        result.push(current_chunk);
    }

    result
}
