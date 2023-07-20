use dotenv::dotenv;
pub use dotenv_codegen::dotenv;
use poise::Event;
pub use poise::{ serenity_prelude::GatewayIntents, PrefixFrameworkOptions };
use radio::player;
use songbird::SerenityInit;

mod prelude;
use prelude::*;

mod commands;
use commands::*;

mod services;
mod radio;
mod utils;
mod render;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let discord_token = dotenv!("DISCORD_TOKEN");

    let intents =
        GatewayIntents::non_privileged() |
        GatewayIntents::GUILDS |
        GatewayIntents::MESSAGE_CONTENT |
        GatewayIntents::DIRECT_MESSAGES |
        GatewayIntents::GUILD_MESSAGES;

    let framework = poise::Framework
        ::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                ping_command::handler(),
                play_command::handler(),
                resume_command::handler(),
                pause_command::handler(),
                playing_command::handler(),
                queue_command::handler(),
                skip_command::handler()
            ],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("+".into()),
                mention_as_prefix: true,
                case_insensitive_commands: false,
                ignore_bots: true,
                ..Default::default()
            },
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(async move {
                    match event {
                        Event::GuildCreate { guild, is_new: _ } => {
                            player::init_player(&guild.id).await;
                        }
                        _ => {}
                    }

                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(discord_token)
        .intents(intents)
        .client_settings(|client| client.register_songbird())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
