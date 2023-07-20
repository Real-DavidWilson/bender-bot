use poise::command;
use crate::prelude::{ Context, Error };
use crate::radio::player;

#[command(rename = "pular", slash_command, prefix_command)]
pub async fn handler(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let guild_id = ctx.guild_id().unwrap();

    let resolved_player = player::resolve(&guild_id).await.expect("Player not found.");
    let mut player = resolved_player.write().await;

    player.skip().await;

    ctx.send(|m| m.reply(true).content("Pulando música...")).await?;

    Ok(())
}
