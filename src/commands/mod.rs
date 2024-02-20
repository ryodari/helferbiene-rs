pub mod ping;

use serenity::{
    all::{CommandInteraction, Context, CreateCommand},
    async_trait,
};

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;

    fn register(&self) -> CreateCommand;
    async fn run(&self, ctx: &Context, command: &CommandInteraction) -> serenity::Result<()>;
}

pub const COMMANDS: &[&dyn Command] = &[&ping::PingCommand];
