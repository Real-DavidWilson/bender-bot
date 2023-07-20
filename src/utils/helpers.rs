use std::time::Duration;

use usvg::{ TreeParsing, TreeTextToPath, Options };

pub fn str_limit(input: String, limit: usize) -> String {
    if input.chars().count() <= limit {
        return input;
    }

    let mut new_str: String = input.chars().take(limit).collect();
    let mut last_char_index = limit;

    while !new_str.is_empty() && !new_str.chars().last().unwrap().is_alphanumeric() {
        new_str.pop();
        last_char_index -= 1;
    }

    if last_char_index > 0 {
        new_str.push_str("...");
    }

    new_str
}

pub fn svg_to_image(input: Vec<u8>) -> Vec<u8> {
    let zoom = 1.0;

    let opt = Options::default();

    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();

    let mut tree = usvg::Tree::from_data(&input, &opt).unwrap();

    tree.convert_text(&fontdb);

    let rtree = resvg::Tree::from_usvg(&tree);

    let pixmap_size = rtree.size.to_int_size().scale_by(zoom).unwrap();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    let render_ts = tiny_skia::Transform::from_scale(zoom, zoom);

    rtree.render(render_ts, &mut pixmap.as_mut());

    pixmap.encode_png().unwrap()
}

pub fn format_duration(duration: Duration) -> String {
    let hours = (duration.as_secs() as u32) / 60 / 60;
    let minutes = ((duration.as_secs() as u32) / 60) % 60;
    let seconds = (duration.as_secs() as u32) % 60;

    if hours > 0 {
        return format!("{}:{:02}:{:02}", hours, minutes, seconds);
    }

    if minutes > 0 {
        return format!("{}:{:02}", minutes, seconds);
    }

    format!("0:{:02}", seconds)
}
