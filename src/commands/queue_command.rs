use poise::command;

use crate::{ prelude::{ Context, Error }, radio::{ self, player } };

#[command(rename = "fila", slash_command, prefix_command)]
pub async fn handler(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let author = ctx.author();

    let resolved_player = player::resolve(&guild_id).await.unwrap();
    let player = resolved_player.read().await;

    let embed_queue = radio::embeds::queue_embed(author, &player);

    ctx.send(|m| {
        m.reply(true).embed(|e| {
            e.clone_from(&embed_queue);

            e
        })
    }).await?;

    Ok(())
}
