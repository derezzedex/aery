use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use futures::FutureExt;
use futures::TryFutureExt;
use tower_service::Service;
use worker::*;

use aery_core as core;
use core::summoner;
use futures::stream;
use futures::StreamExt;
use std::cmp::Reverse;

use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};

mod error;
use error::Error;

mod cache;

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_ansi(false) // Only partially supported across JavaScript runtimes
        .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

pub type Result<T> = std::result::Result<T, Error>;

fn router(env: Env) -> Result<Router> {
    let api_key = env.secret("RGAPI_KEY")?.to_string();

    let summoner_kv = env.kv("summoners")?;
    let matches_kv = env.kv("matches")?;

    let router = Router::new()
        .route(
            "/summoner/:region/:riot_id",
            get(fetch_summoner).with_state((api_key.clone(), summoner_kv)),
        )
        .route(
            "/matches/:puuid",
            get(fetch_matches).with_state((api_key, matches_kv)),
        );

    Ok(router)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    router(env)
        .map_err(Error::from_string)?
        .call(req)
        .await
        .map_err(Error::infallible)
}

#[axum::debug_handler]
#[worker::send]
async fn fetch_summoner(
    Path((region, name)): Path<(String, String)>,
    State((api_key, kv)): State<(String, kv::KvStore)>,
) -> Result<Json<summoner::Data>> {
    let client = core::Client::new(api_key);
    let region = core::Region::from(region);
    let riot_id = name.replace("-", "#");

    if let Ok(Some(data)) = cache::get(&kv, &riot_id).await {
        return Ok(axum::Json(data));
    }

    let Ok(summoner) = core::Summoner::from_name(client.clone(), riot_id.clone(), region).await
    else {
        return Err(Error::from_string("Summoner not found!"));
    };

    let Ok(leagues) = summoner
        .leagues(&client, region)
        .await
        .map(|leagues| leagues.collect::<Vec<_>>())
    else {
        return Err(Error::from_string("Failed to fetch summoner leagues."));
    };

    let mut games: Vec<core::Game> = stream::iter(summoner.matches(&client, 0..10, None).await)
        .flat_map(|game_ids| {
            stream::iter(game_ids).filter_map(|id| {
                core::Game::from_id(&client, id)
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
pub struct Matches(Vec<core::Game>);

#[axum::debug_handler]
#[worker::send]
async fn fetch_matches(
    Path(puuid): Path<String>,
    State((api_key, kv)): State<(String, kv::KvStore)>,
) -> Result<Json<Matches>> {
    let client = core::Client::new(api_key);

    if let Ok(Some(matches)) = cache::get(&kv, &puuid).await {
        return Ok(axum::Json(matches));
    }

    let mut games: Vec<core::Game> =
        stream::iter(summoner::matches(&puuid, &client, 0..10, None).await)
            .flat_map(|game_ids| {
                stream::iter(game_ids).filter_map(|id| {
                    core::Game::from_id(&client, id)
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
