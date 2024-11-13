use crate::{Error, Result};
use axum::extract::{Path, State};
use axum::response::Response;

#[worker::send]
#[axum::debug_handler]
pub async fn fetch(
    Path(version): Path<String>,
    State(env): State<worker::Env>,
) -> Result<Response> {
    console_error_panic_hook::set_once();
    let bucket = env.bucket("assets").map_err(Error::from_string)?;
    let body = bucket
        .get(version)
        .execute()
        .await
        .map_err(Error::from_string)?
        .ok_or(Error::not_found())?
        .body()
        .ok_or(Error::not_found())?
        .response_body()
        .map_err(Error::from_string)?;

    Ok(worker::Response::from_body(body)
        .map_err(Error::from_string)?
        .with_status(200)
        .into())
}
