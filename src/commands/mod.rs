pub mod ping;
pub mod server_info;

use serenity::{
    all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse},
    async_trait,
};

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;

    fn register(&self) -> CreateCommand;
    async fn run(&self, ctx: &Context, command: &CommandInteraction) -> serenity::Result<Option<CreateInteractionResponse>>;
}

pub const COMMANDS: &[&dyn Command] = &[&ping::PingCommand, &server_info::ServerInfoCommand];
