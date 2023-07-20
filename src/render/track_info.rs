use std::time::Duration;

use tera::Context;

use crate::utils::helpers;

use super::TEMPLATES;

pub struct TrackInfoDrawOptions {
    pub title: String,
    pub artist: String,
    pub current_duration: Duration,
    pub total_duration: Duration,
    pub thumb: String,
    pub playing: bool,
}

pub fn draw(options: TrackInfoDrawOptions) -> Vec<u8> {
    let current_num = options.current_duration.as_secs() as f64;
    let total_num = options.total_duration.as_secs() as f64;

    let mut progress = (current_num / total_num) * 100.0;

    if progress > 100.0 {
        progress = 100.0;
    }

    let mut context = Context::new();

    let title = options.title.as_bytes();
    let title = String::from_utf8_lossy(title);

    let artist = options.artist.as_bytes();
    let artist = String::from_utf8_lossy(artist);

    context.insert("title", &title);
    context.insert("artist", &artist);
    context.insert("thumb", &options.thumb);
    context.insert("current_duration", &helpers::format_duration(options.current_duration));
    context.insert("total_duration", &helpers::format_duration(options.total_duration));
    context.insert("playing", &options.playing);
    context.insert("progress", &progress);

    let result = TEMPLATES.render("track-info.html", &context).unwrap();

    let svg = helpers::svg_to_image(Vec::from(result));

    svg
}
