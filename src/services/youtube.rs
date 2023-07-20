use std::{ time::Duration, str::FromStr };

use base64::Engine;
use rusty_ytdl::search::{ YouTube, SearchOptions, SearchType, SearchResult };
use songbird::input::{ Input, Metadata };

use crate::prelude::Error;

pub async fn stream(uri: &String) -> Result<Input, Error> {
    let result = songbird::ytdl(uri).await.unwrap();

    return Ok(result);
}

pub async fn search(uri: &String) -> Result<Metadata, Error> {
    let youtube = YouTube::new().unwrap();
    let mut metadata = Metadata::default();

    let mut search_options = SearchOptions::default();

    search_options.limit = 1;
    search_options.search_type = SearchType::Video;

    let result = youtube
        .search_one(uri, Some(&search_options)).await
        .expect("Falha ao pesquisar no youtube.");

    if result.is_none() {
        return Err(Error::from("Música não encontrada."));
    }

    let result = result.unwrap();

    if let SearchResult::Video(video) = result {
        metadata.artist = Some(video.channel.name.clone());
        metadata.channel = Some(video.channel.name);
        metadata.source_url = Some(video.url);
        metadata.thumbnail = Some(video.thumbnails.first().as_ref().unwrap().url.clone());
        metadata.date = video.uploaded_at;
        metadata.title = Some(video.title);
        metadata.duration = Some(Duration::from_millis(video.duration));

        return Ok(metadata);
    }

    return Err(Error::from("Não foi possível buscar a música."));
}

pub async fn fetch_thumb(url: &String) -> Option<String> {
    let mut url = reqwest::Url::from_str(url).unwrap();

    url.query_pairs_mut().clear();

    let res = reqwest::get(url).await.unwrap();

    let data = res.bytes().await.unwrap();

    let base64_data = base64::engine::general_purpose::STANDARD_NO_PAD.encode(data);
    let base64_str = format!("data:image/jpeg;base64,{}", base64_data);

    Some(base64_str)
}
