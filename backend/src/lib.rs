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
use futures::stream;
use futures::StreamExt;
use std::cmp::Reverse;

#[derive(Debug)]
pub struct ErrWrapper(worker::Error);

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

fn router(api_key: String) -> Router {
    Router::new()
        .route("/summoner/:region/:riot_id", get(fetch_summoner))
        .with_state(api_key)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    let api_key = env.secret("RGAPI_KEY")?;

    router(api_key.to_string())
        .call(req)
        .await
        .map_err(|_| ErrWrapper(worker::Error::Infallible))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Data {
    summoner: core::Summoner,
    leagues: Vec<core::summoner::League>,
    games: Vec<core::Game>,
}

#[axum::debug_handler]
#[worker::send]
async fn fetch_summoner(
    Path((region, name)): Path<(String, String)>,
    State(api_key): State<String>,
) -> Result<Json<Data>> {
    let client = core::Client::new(api_key);
    let region = core::Region::from(region);
    let riot_id = name.replace("-", "#");

    let Ok(summoner) = core::Summoner::from_name(client.clone(), riot_id, region).await else {
        return Err(worker::Error::RustError(String::from("Summoner not found!")).into());
    };

    let Ok(leagues) = summoner
        .leagues(&client, region)
        .await
        .map(|leagues| leagues.collect::<Vec<_>>())
    else {
        return Err(
            worker::Error::RustError(String::from("Failed to fetch summoner leagues.")).into(),
        );
    };

    let mut games: Vec<core::Game> = stream::iter(summoner.matches(&client, 0..10, None).await)
        .flat_map(|game_ids| {
            stream::iter(game_ids).filter_map(|id| {
                core::Game::from_id(&client, id)
                    .map_err(|e| worker::Error::RustError(e.to_string()).into())
                    .map(Result::ok)
            })
        })
        .collect()
        .await;

    games.sort_unstable_by_key(|game| Reverse(game.created_at()));

    Ok(axum::Json(Data {
        summoner,
        leagues,
        games,
    }))
}
