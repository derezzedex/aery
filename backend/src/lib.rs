use axum::routing::get;
use axum::Router;
use tower_service::Service;
use worker::*;

use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};

mod error;
use error::Error;

mod assets;
mod cache;
mod game;
mod summoner;

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
            get(summoner::fetch).with_state((api_key.clone(), summoner_kv)),
        )
        .route(
            "/matches/:puuid",
            get(summoner::matches).with_state((api_key, matches_kv)),
        )
        .route("/assets/:version", get(assets::fetch).with_state(env));

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
