use dotenv_codegen::dotenv;
use poise::{serenity_prelude::GatewayIntents, PrefixFrameworkOptions};

use commands::*;

mod commands;

pub struct Data {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

const DISCORD_TOKEN: &'static str = dotenv!("DISCORD_TOKEN");

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping::command()],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("+".into()),
                mention_as_prefix: true,
                case_insensitive_commands: false,
                ignore_bots: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .token(DISCORD_TOKEN)
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
