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

    async fn run(
        &self,
        _ctx: &Context,
        _command: &CommandInteraction,
    ) -> serenity::Result<Option<CreateInteractionResponse>> {
        let data = CreateInteractionResponseMessage::new().content("Pong!");

        Ok(Some(CreateInteractionResponse::Message(data)))
    }
}
