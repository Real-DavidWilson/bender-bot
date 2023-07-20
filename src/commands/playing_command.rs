use std::borrow::Cow;
use poise::command;
use poise::serenity_prelude::AttachmentType;
use songbird::tracks::PlayMode;

use crate::prelude::{ Context, Error };
use crate::radio::player;
use crate::render::track_info::{ self, TrackInfoDrawOptions };
use crate::services::youtube;

#[command(rename = "tocando", slash_command, prefix_command)]
pub async fn handler(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let guild_id = ctx.guild_id().unwrap();

    let resolved_player = player::resolve(&guild_id).await.expect("Player not found.");
    let player = resolved_player.read().await;

    if !player.is_playing().await {
        ctx.send(|m| m.reply(true).content("Não há nada tocando no momento.")).await?;
    }

    let player_track = player.current_input.as_ref().unwrap();
    let track_handle = player.track_handle.as_ref().unwrap();
    let track_info = track_handle.get_info().await.clone().unwrap();

    let metadata = player_track.metadata.clone();

    let title = metadata.title.as_ref().unwrap();

    let artist = metadata.artist.as_ref().unwrap();
    let thumb = metadata.thumbnail.as_ref().unwrap();
    let total_duration = metadata.duration.as_ref().unwrap();
    let current_duration = track_info.position;
    let playing = track_info.playing == PlayMode::Play;

    let thumb = youtube::fetch_thumb(thumb).await.unwrap();

    let draw_options = TrackInfoDrawOptions {
        title: title.to_owned(),
        artist: artist.to_owned(),
        total_duration: total_duration.to_owned(),
        thumb,
        current_duration,
        playing,
    };

    let chart = track_info::draw(draw_options);

    let attachment = AttachmentType::Bytes { data: Cow::from(chart), filename: "chart.png".into() };

    ctx.send(|m| m.reply(true).attachment(attachment)).await?;

    Ok(())
}
