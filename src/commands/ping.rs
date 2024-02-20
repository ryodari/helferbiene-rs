use serenity::{
    all::{
        CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    async_trait,
};

use super::Command;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("A ping command")
    }

    async fn run(&self, ctx: &Context, command: &CommandInteraction) -> serenity::Result<()> {
        let data = CreateInteractionResponseMessage::new().content("Pong!");

        let builder = CreateInteractionResponse::Message(data);

        command.create_response(&ctx.http, builder).await?;

        Ok(())
    }
}
