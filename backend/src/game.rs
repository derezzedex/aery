use riven::consts::RegionalRoute;

use crate::{Error, Result};
use aery_core::game;
pub use aery_core::game::{Game, Id, Queue, Timeline};
use aery_core::Client;

pub async fn fetch(client: &Client, id: Id) -> Result<Game> {
    client
        .as_ref()
        .match_v5()
        .get_match(RegionalRoute::AMERICAS, id.as_ref())
        .await
        .map_err(Error::from_string)
        .and_then(|game| game.map(Game).ok_or(Error::not_found()))
}

#[allow(dead_code)]
pub async fn fetch_timeline(client: &Client, id: Id) -> Result<Timeline> {
    client
        .as_ref()
        .match_v5()
        .get_timeline(RegionalRoute::AMERICAS, id.as_ref())
        .await
        .map_err(Error::from_string)
        .and_then(|timeline| {
            timeline
                .map(|tl| {
                    let events = tl
                        .info
                        .frames
                        .into_iter()
                        .flat_map(|frame| frame.events.into_iter())
                        .map(game::Event)
                        .collect::<Vec<_>>();
                    game::Timeline(events)
                })
                .ok_or(Error::not_found())
        })
}
