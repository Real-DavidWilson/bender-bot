use std::{ collections::{ HashMap, VecDeque }, sync::Arc };

use lazy_static::lazy_static;
use poise::serenity_prelude::{ Context, GuildId, ChannelId, User };
use songbird::{ create_player, tracks::{ PlayMode, TrackHandle }, TrackEvent, input::Metadata };
use tokio::sync::RwLock;

use crate::services::youtube;

use super::handler::TrackEndHandler;

lazy_static! {
    static ref PLAYERS: RwLock<HashMap<u64, Arc<RwLock<Player>>>> = RwLock::new(HashMap::new());
}

#[derive(Clone)]
pub enum LoopState {
    Finite(u64),
    Inifnite,
}

#[derive(Clone)]
pub struct PlayerTrack {
    pub ctx: Context,
    pub metadata: Metadata,
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub author: User,
}

#[derive(Clone, Default)]
pub struct Player {
    pub guild_id: GuildId,
    pub tracklist: VecDeque<PlayerTrack>,
    pub track_handle: Option<TrackHandle>,
    pub current_input: Option<PlayerTrack>,
    pub loop_state: Option<LoopState>,
}

impl Player {
    fn new(guild_id: GuildId) -> Self {
        Self {
            guild_id,
            ..Default::default()
        }
    }

    pub async fn add(
        &mut self,
        ctx: &Context,
        guild_id: &GuildId,
        channel_id: &ChannelId,
        author: &User,
        uri: String
    ) -> Option<Metadata> {
        let metadata = youtube::search(&uri).await.unwrap();

        let input = PlayerTrack {
            ctx: ctx.clone(),
            metadata: metadata.clone(),
            guild_id: guild_id.clone(),
            channel_id: channel_id.clone(),
            author: author.clone(),
        };

        self.tracklist.push_back(input);

        let guild_id = guild_id.clone();

        tokio::spawn(async move {
            let resolved_player = resolve(&guild_id).await.unwrap();
            let mut player = resolved_player.write().await;
            player.update().await;
        });

        Some(metadata)
    }

    pub async fn skip(&mut self) {
        if self.track_handle.is_none() {
            return;
        }

        let track_handle = self.track_handle.as_ref().unwrap();
        let result = track_handle.stop();

        if result.is_ok() {
            return;
        }

        self.track_handle = None;
    }

    pub async fn is_playing(&self) -> bool {
        if self.track_handle.is_none() {
            return false;
        }

        let track_handle = self.track_handle.as_ref().unwrap();
        let track_info = track_handle.get_info().await;

        if track_info.is_err() {
            return false;
        }

        let playing = track_info.unwrap().playing;

        if playing == PlayMode::End || playing == PlayMode::Stop {
            return false;
        }

        return true;
    }

    pub async fn update(&mut self) {
        if self.is_playing().await {
            return;
        }

        let next_input = self.tracklist.pop_front();

        if next_input.is_none() {
            return self.reset();
        }

        let next_input = next_input.unwrap();
        let track_handle = Player::raw_play(&next_input).await.unwrap();

        self.track_handle = Some(track_handle);
        self.current_input = Some(next_input);
    }

    fn reset(&mut self) {
        self.current_input = None;
        self.track_handle = None;
        self.loop_state = None;
    }

    async fn raw_play(input: &PlayerTrack) -> Option<TrackHandle> {
        let ctx = &input.ctx;
        let guild_id = &input.guild_id;
        let author = &input.author;

        let url = input.metadata.source_url.as_ref().unwrap();
        let guild = guild_id.to_guild_cached(&ctx.cache).unwrap();

        let voice_channel_id = guild.voice_states
            .get(&author.id)
            .and_then(|voice_state| voice_state.channel_id)?;

        let manager = songbird::get(&ctx).await.unwrap();

        let _ = manager.join(guild_id.0, voice_channel_id).await;

        let mut handler = manager.get(guild_id.0)?.lock_owned().await;

        let source = youtube::stream(url).await.unwrap();
        let (track, track_handle) = create_player(source);

        handler.play_only(track);
        handler.deafen(true).await.unwrap();

        track_handle
            .add_event(songbird::Event::Track(TrackEvent::End), TrackEndHandler {
                guild_id: guild_id.clone(),
            })
            .unwrap();

        Some(track_handle)
    }
}

pub async fn init_player(guild_id: &GuildId) {
    let mut players = PLAYERS.write().await;

    if players.get(&guild_id.0).is_some() {
        return;
    }

    let player = Player::new(guild_id.clone());

    players.insert(guild_id.0, Arc::new(RwLock::new(player)));
}

pub async fn resolve(guild_id: &GuildId) -> Option<Arc<RwLock<Player>>> {
    let players = PLAYERS.read().await;
    let player = players.get(&guild_id.0)?;

    Some(player.clone())
}
