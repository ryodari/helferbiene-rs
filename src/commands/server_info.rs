use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
        CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
        EditInteractionResponse, ResolvedOption, ResolvedValue,
    },
    async_trait,
};

use crate::minecraft;

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

        let host = match options.first() {
            Some(ResolvedOption {
                value: ResolvedValue::String(host),
                ..
            }) => host,
            _ => {
                return Ok(Some(CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Please provide a host"),
                )))
            }
        };

        let port: u16 = match options.iter().find(|o| o.name == "port") {
            Some(ResolvedOption {
                value: ResolvedValue::Integer(port),
                ..
            }) => *port as u16,
            _ => 25565,
        };

        // defer
        command.defer(&ctx.http).await?;

        let client = minecraft::client::Client::new(host.to_string(), port).await?;
        let mut info = client.status().await?;

        info.favicon = Some("ignored".into());

        let pretty = serde_json::to_string_pretty(&info).unwrap();

        let embed = CreateEmbed::new()
            .title(format!("{}:{}", host, port))
            .description(format!("```json\n{}```", pretty));

        let data = EditInteractionResponse::new().add_embed(embed);

        command.edit_response(&ctx.http, data).await?;

        return Ok(None);
    }
}
