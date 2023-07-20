use poise::command;

use crate::prelude::{ Context, Error };
use crate::radio::player;

#[command(rename = "pausar", slash_command, prefix_command)]
pub async fn handler(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let guild_id = ctx.guild_id().unwrap();

    let resolved_player = player::resolve(&guild_id).await.expect("Player not found.");
    let player = resolved_player.write().await;

    let track_handle = player.track_handle.as_ref().unwrap();

    track_handle.pause().unwrap();

    ctx.send(|m| m.reply(true).content("A rádio foi pausada.")).await?;

    Ok(())
}
