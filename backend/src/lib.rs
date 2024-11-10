use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response;
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

#[derive(Debug)]
pub struct ErrWrapper(worker::Error);

impl ErrWrapper {
    fn from_string(value: impl ToString) -> Self {
        ErrWrapper(worker::Error::RustError(value.to_string()))
    }
}

impl std::fmt::Display for ErrWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ErrWrapper {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<worker::Error> for ErrWrapper {
    fn from(value: worker::Error) -> Self {
        ErrWrapper(value)
    }
}

impl response::IntoResponse for ErrWrapper {
    fn into_response(self) -> response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ErrWrapper>;

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
        .map_err(ErrWrapper::from_string)?
        .call(req)
        .await
        .map_err(|_| ErrWrapper(worker::Error::Infallible))
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

    match kv
        .get(&riot_id)
        .text()
        .await
        .map_err(ErrWrapper::from_string)?
    {
        Some(text) => {
            let data = serde_json::from_str(&text).map_err(ErrWrapper::from_string)?;

            tracing::debug!("returning cached summoner");
            return Ok(axum::Json(data));
        }
        None => tracing::debug!("summoner not found in KV"),
    }

    let Ok(summoner) = core::Summoner::from_name(client.clone(), riot_id.clone(), region).await
    else {
        return Err(ErrWrapper::from_string("Summoner not found!"));
    };

    let Ok(leagues) = summoner
        .leagues(&client, region)
        .await
        .map(|leagues| leagues.collect::<Vec<_>>())
    else {
        return Err(ErrWrapper::from_string("Failed to fetch summoner leagues."));
    };

    let mut games: Vec<core::Game> = stream::iter(summoner.matches(&client, 0..10, None).await)
        .flat_map(|game_ids| {
            stream::iter(game_ids).filter_map(|id| {
                core::Game::from_id(&client, id)
                    .map_err(ErrWrapper::from_string)
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

    let kv_data = serde_json::to_string(&data).map_err(ErrWrapper::from_string)?;

    kv.put(&riot_id, kv_data)
        .inspect_err(|e| tracing::error!("put failed: {e}"))
        .map_err(ErrWrapper::from_string)?
        .execute()
        .await
        .inspect_err(|e| tracing::error!("execute failed: {e}"))
        .map_err(ErrWrapper::from_string)?;

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

    match kv
        .get(&puuid)
        .text()
        .await
        .map_err(ErrWrapper::from_string)?
    {
        Some(text) => {
            let matches = serde_json::from_str(&text).map_err(ErrWrapper::from_string)?;

            tracing::debug!("returning cached summoner");
            return Ok(axum::Json(matches));
        }
        None => tracing::debug!("summoner not found in KV"),
    }

    let mut games: Vec<core::Game> =
        stream::iter(summoner::matches(&puuid, &client, 0..10, None).await)
            .flat_map(|game_ids| {
                stream::iter(game_ids).filter_map(|id| {
                    core::Game::from_id(&client, id)
                        .map_err(ErrWrapper::from_string)
                        .map(Result::ok)
                })
            })
            .collect()
            .await;

    games.sort_unstable_by_key(|game| Reverse(game.created_at()));

    let matches = Matches(games);

    let kv_data = serde_json::to_string(&matches).map_err(ErrWrapper::from_string)?;

    kv.put(&puuid, kv_data)
        .inspect_err(|e| tracing::error!("put failed: {e}"))
        .map_err(ErrWrapper::from_string)?
        .execute()
        .await
        .inspect_err(|e| tracing::error!("execute failed: {e}"))
        .map_err(ErrWrapper::from_string)?;

    Ok(axum::Json(matches))
}
