use poise::command;

use crate::prelude::{ Context, Error };
use crate::radio::player;

#[command(rename = "tocar", slash_command, prefix_command)]
pub async fn handler(
    ctx: Context<'_>,
    #[description = "URL ou nome da música."] #[rest = true] uri: String
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let guild_id = ctx.guild_id().unwrap();
    let channel_id = ctx.channel_id();
    let author = ctx.author();

    let resolved_player = player::resolve(&guild_id).await.expect("Player not found.");
    let mut player = resolved_player.write().await;

    player.add(ctx.serenity_context(), &guild_id, &channel_id, author, uri).await;

    ctx.send(|m| m.reply(true).content("Sua música foi adicionada à rádio.")).await?;

    Ok(())
}
