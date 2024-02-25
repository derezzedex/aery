pub mod league;
pub use league::{Division, League, Tier};

use crate::game;
use crate::Client;
use riven::consts::{PlatformRoute, RegionalRoute};

#[derive(Debug, Clone)]
pub struct InternalApiError(String);

#[derive(Debug, Clone, thiserror::Error)]
pub enum RequestError {
    #[error("not found")]
    NotFound,
    #[error("request failed")]
    RequestFailed(InternalApiError),
}

#[derive(Debug, Clone)]
pub struct RiotId {
    pub name: Option<String>, // 3~16 chars
    pub tagline: String,      // 3~5 chars
}

#[derive(Debug, Clone)]
pub struct Summoner(riven::models::summoner_v4::Summoner);

impl Summoner {
    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn puuid(&self) -> &str {
        &self.0.puuid
    }

    pub fn level(&self) -> u32 {
        self.0.summoner_level as u32
    }

    pub fn icon_id(&self) -> i32 {
        self.0.profile_icon_id
    }

    pub async fn from_name(client: Client, name: String) -> Result<Self, RequestError> {
        client
            .as_ref()
            .summoner_v4()
            .get_by_summoner_name(PlatformRoute::BR1, &name)
            .await
            .map_err(|error| RequestError::RequestFailed(InternalApiError(error.to_string())))
            .and_then(|summoner| summoner.map(Summoner).ok_or(RequestError::NotFound))
    }

    pub async fn matches(
        &self,
        client: &Client,
        range: std::ops::Range<u32>,
        queue: impl Into<Option<game::Queue>>,
    ) -> Result<impl Iterator<Item = game::Id>, RequestError> {
        client
            .as_ref()
            .match_v5()
            .get_match_ids_by_puuid(
                RegionalRoute::AMERICAS,
                &self.0.puuid,
                Some((range.end - range.start) as i32),
                None,
                queue.into().map(game::Queue::into),
                None,
                Some(range.start as i32),
                None,
            )
            .await
            .map_err(|error| RequestError::RequestFailed(InternalApiError(error.to_string())))
            .map(|list| list.into_iter().filter_map(|s| s.try_into().ok()))
    }

    pub async fn leagues(
        &self,
        client: &Client,
    ) -> Result<impl Iterator<Item = League>, RequestError> {
        client
            .as_ref()
            .league_v4()
            .get_league_entries_for_summoner(PlatformRoute::BR1, &self.0.id)
            .await
            .map_err(|error| RequestError::RequestFailed(InternalApiError(error.to_string())))
            .map(|leagues| leagues.into_iter().map(League))
    }
}
