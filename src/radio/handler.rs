use poise::{ async_trait, serenity_prelude::GuildId };
use songbird::{ Event, EventContext, EventHandler };

use super::player;

pub struct TrackEndHandler {
    pub guild_id: GuildId,
}

#[async_trait]
impl EventHandler for TrackEndHandler {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let guild_id = self.guild_id.clone();

        tokio::spawn(async move {
            let resolved_player = player::resolve(&guild_id).await.unwrap();
            let mut player = resolved_player.write().await;

            player.update().await;
        });

        None
    }
}
