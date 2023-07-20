use poise::serenity_prelude::{ CreateEmbed, CreateMessage, User };

use crate::utils::helpers;

use super::player::{ PlayerTrack, Player };

pub fn track_start_embed(item: PlayerTrack) -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    let metadata = item.metadata.clone();
    let author = item.author.clone();

    let title = metadata.title.as_ref().unwrap();
    let channel = metadata.artist.as_ref().unwrap();
    let duration = metadata.duration.as_ref().unwrap();
    let url = metadata.source_url.as_ref().unwrap();
    let thumb = metadata.thumbnail.as_ref().unwrap();

    embed
        .color(0xc3e2e1)
        .author(|a| {
            a.name(author.clone().name).icon_url(author.avatar_url().unwrap_or_else(|| "".into()))
        })
        .thumbnail(
            "https://cdn.icon-icons.com/icons2/1429/PNG/512/icon-robots-16_98547.png".to_string()
        )
        .image(thumb)
        .description("Rádio do Bender.")
        .field("", "", false)
        .field("", "", false)
        .field("Titulo", title, true)
        .field("Canal", channel, true)
        .field("", "", false)
        .field("Duração", helpers::format_duration(duration.to_owned()), true)
        .field("", "", false)
        .field("URL", url, false)
        .footer(|f|
            f
                .icon_url(
                    "https://images.squarespace-cdn.com/content/v1/587d4a02bebafb893ba07d90/1484886557050-V261JTTHHGX0O3KHW5OX/ui-ux-playlist-gen-icon.png"
                )
                .text("Rádio - BenderRadio")
        );

    return embed;
}

pub fn queue_embed(author: &User, player: &Player) -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    let current_track = &player.current_input;
    let tracklist = &player.tracklist;

    embed
        .colour(0xc3e2e1)
        .image("https://i.gifer.com/1bJC.gif")
        .author(|a| {
            a.name(author.name.clone()).icon_url(author.avatar_url().unwrap_or_default())
        });

    if current_track.is_none() {
        embed.field("Música Atual", "Nenhuma", false);
    }

    if let Some(current_track) = current_track {
        let metadata = current_track.metadata.clone();

        let title = metadata.title.as_ref().unwrap();
        let url = metadata.source_url.as_ref().unwrap();
        let channel = metadata.artist.as_ref().unwrap();

        let current_track_str = format!(
            "[__{}__]({}) | {}",
            helpers::str_limit(title.to_owned(), 32),
            url,
            channel
        );

        embed.field("Música Atual", current_track_str, false);
    }

    if tracklist.is_empty() {
        embed.field("Músicas", "Nenhuma", false);
    }

    if !tracklist.is_empty() {
        let mut tracklist_str = String::new();

        for (i, track) in tracklist.iter().enumerate() {
            let metadata = track.metadata.clone();

            let title = metadata.title.as_ref().unwrap();
            let url = metadata.source_url.as_ref().unwrap();
            let channel = metadata.artist.as_ref().unwrap();

            tracklist_str.push_str(
                format!(
                    "{} - [__{}__]({}) | {}",
                    i + 1,
                    helpers::str_limit(title.to_owned(), 32),
                    url,
                    channel
                ).as_str()
            );

            tracklist_str.push_str("\n".into());
        }

        embed.field("Músicas", tracklist_str, false);
    }

    embed.footer(|f|
        f
            .icon_url(
                "https://images.squarespace-cdn.com/content/v1/587d4a02bebafb893ba07d90/1484886557050-V261JTTHHGX0O3KHW5OX/ui-ux-playlist-gen-icon.png"
            )
            .text("Fila - BenderRadio")
    );

    return embed;
}

// pub fn message_queue<'a>(info: PlayerInfo) -> CreateMessage<'a> {
//     let mut message = CreateMessage::<'a>::default();
//     message.embed(|e| {
//         if some {

//         }

//         e.color(0xc3e2e1)
//         .thumbnail("https://thestickerboy.com/wp-content/uploads/2019/11/futurama_tv_show_cartoon_bender_sticker_16__82182.png")
//         .author(|a| {
//             a.name(ctx.author().name.clone())
//                 .icon_url(ctx.author().avatar_url().unwrap())
//         })
//         .field("Música Atual", "[**Volare** | Dean Martin](https://youtube.com)", false)
//         .field("Músicas - 1/3", queue_str, false)
//         .image("https://i.gifer.com/1bJC.gif")
//         .footer(|f| {
//             f.icon_url(
//                 "https://images.squarespace-cdn.com/content/v1/587d4a02bebafb893ba07d90/1484886557050-V261JTTHHGX0O3KHW5OX/ui-ux-playlist-gen-icon.png",
//             )
//             .text("Fila Musical - BenderBot")
//         });

//         e
//     });

//     return message;
// }

pub fn message_queue_added<'a>() -> CreateMessage<'a> {
    let message = CreateMessage::default();

    return message;
}

pub fn message_queue_end<'a>() -> CreateMessage<'a> {
    let message = CreateMessage::default();

    return message;
}
