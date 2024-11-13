use axum::extract::{Path, State};
use axum::Json;
use futures::stream;
use futures::FutureExt;
use futures::StreamExt;
use futures::TryFutureExt;
use riven::consts::RegionalRoute;
use std::cmp::Reverse;
use worker::kv;

use crate::cache;
use crate::game;
use crate::{Error, Result};
use aery_core::{account, summoner};
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

    let Ok(summoner) = summoner(client.clone(), riot_id.clone(), region).await else {
        return Err(Error::from_string("Summoner not found!"));
    };

    let Ok(leagues) = leagues(&summoner.raw.id, &client, region)
        .await
        .map(|leagues| leagues.collect::<Vec<_>>())
    else {
        return Err(Error::from_string("Failed to fetch summoner leagues."));
    };

    let mut games: Vec<Game> = stream::iter(games(summoner.puuid(), &client, 0..10, None).await)
        .flat_map(|game_ids| {
            stream::iter(game_ids).filter_map(|id| {
                game::fetch(&client, id)
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

    let mut games: Vec<Game> = stream::iter(games(&puuid, &client, 0..10, None).await)
        .flat_map(|game_ids| {
            stream::iter(game_ids).filter_map(|id| {
                game::fetch(&client, id)
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

pub async fn summoner(client: Client, name: String, region: Region) -> Result<Summoner> {
    let mut account_id = name.split("#");

    let (game_name, tag_line) = (
        account_id.next().ok_or(Error::not_found())?,
        account_id.next().ok_or(Error::not_found())?,
    );

    tracing::info!("Requesting account: {game_name}#{tag_line}");

    let riot_id = account::RiotId::new(game_name, tag_line);

    let account = client
        .as_ref()
        .account_v1()
        .get_by_riot_id(RegionalRoute::AMERICAS, game_name, tag_line)
        .await
        .map_err(Error::from_string)?
        .ok_or(Error::not_found())?;

    client
        .as_ref()
        .summoner_v4()
        .get_by_puuid(region.0, &account.puuid)
        .await
        .map_err(Error::from_string)
        .map(|summoner| Summoner {
            riot_id,
            raw: summoner,
        })
}

pub async fn leagues(
    summoner_id: &str,
    client: &Client,
    region: Region,
) -> Result<impl Iterator<Item = summoner::League>> {
    client
        .as_ref()
        .league_v4()
        .get_league_entries_for_summoner(region.0, summoner_id)
        .await
        .map_err(Error::from_string)
        .map(|leagues| leagues.into_iter().map(summoner::League))
}

pub async fn games(
    puuid: &str,
    client: &Client,
    range: std::ops::Range<u32>,
    queue: impl Into<Option<game::Queue>>,
) -> Result<impl Iterator<Item = game::Id>> {
    client
        .as_ref()
        .match_v5()
        .get_match_ids_by_puuid(
            RegionalRoute::AMERICAS,
            puuid,
            Some((range.end - range.start) as i32),
            None,
            queue.into().map(game::Queue::into),
            None,
            Some(range.start as i32),
            None,
        )
        .await
        .map_err(Error::from_string)
        .map(|list| list.into_iter().filter_map(|s| s.try_into().ok()))
}
