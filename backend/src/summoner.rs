use axum::extract::{Path, State};
use axum::Json;
use futures::stream;
use futures::FutureExt;
use futures::StreamExt;
use futures::TryFutureExt;
use std::cmp::Reverse;
use worker::kv;

use crate::cache;
use crate::{Error, Result};
use aery_core::summoner;
use aery_core::{Client, Game, Region, Summoner};

#[worker::send]
#[axum::debug_handler]
pub async fn fetch(
    Path((region, name)): Path<(String, String)>,
    State((api_key, kv)): State<(String, kv::KvStore)>,
) -> Result<Json<summoner::Data>> {
    let client = Client::new(api_key);
    let region = Region::from(region);
    let riot_id = name.replace("-", "#");

    if let Ok(Some(data)) = cache::get(&kv, &riot_id).await {
        return Ok(axum::Json(data));
    }

    let Ok(summoner) = Summoner::from_name(client.clone(), riot_id.clone(), region).await else {
        return Err(Error::from_string("Summoner not found!"));
    };

    let Ok(leagues) = summoner
        .leagues(&client, region)
        .await
        .map(|leagues| leagues.collect::<Vec<_>>())
    else {
        return Err(Error::from_string("Failed to fetch summoner leagues."));
    };

    let mut games: Vec<Game> = stream::iter(summoner.matches(&client, 0..10, None).await)
        .flat_map(|game_ids| {
            stream::iter(game_ids).filter_map(|id| {
                Game::from_id(&client, id)
                    .map_err(Error::from_string)
                    .map(Result::ok)
            })
        })
        .collect()
        .await;

    games.sort_unstable_by_key(|game| Reverse(game.created_at()));

    let data = summoner::Data {
        summoner,
        leagues,
        games,
    };

    let _ = cache::insert(&kv, &riot_id, &data).await;

    Ok(axum::Json(data))
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Matches(Vec<Game>);

#[worker::send]
#[axum::debug_handler]
pub async fn matches(
    Path(puuid): Path<String>,
    State((api_key, kv)): State<(String, kv::KvStore)>,
) -> Result<Json<Matches>> {
    let client = Client::new(api_key);

    if let Ok(Some(matches)) = cache::get(&kv, &puuid).await {
        return Ok(axum::Json(matches));
    }

    let mut games: Vec<Game> = stream::iter(summoner::matches(&puuid, &client, 0..10, None).await)
        .flat_map(|game_ids| {
            stream::iter(game_ids).filter_map(|id| {
                Game::from_id(&client, id)
                    .map_err(Error::from_string)
                    .map(Result::ok)
            })
        })
        .collect()
        .await;

    games.sort_unstable_by_key(|game| Reverse(game.created_at()));

    let matches = Matches(games);

    let _ = cache::insert(&kv, &puuid, &matches).await;

    Ok(axum::Json(matches))
}
