use poise::command;

use crate::{Context, Error};

#[command(rename = "ping", slash_command, prefix_command = true)]
pub async fn command(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;

    Ok(())
}
