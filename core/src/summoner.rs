use crate::game;
use crate::Client;
use crate::Queue;
use riven::consts::{PlatformRoute, RegionalRoute};

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("not found")]
    NotFound,
    #[error(transparent)]
    RequestFailed(#[from] riven::RiotApiError),
}

#[derive(Debug)]
pub struct Summoner(riven::models::summoner_v4::Summoner);

impl Summoner {
    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn puuid(&self) -> &str {
        &self.0.puuid
    }

    pub async fn from_name(client: &Client, name: &str) -> Result<Self, RequestError> {
        client
            .0
            .summoner_v4()
            .get_by_summoner_name(PlatformRoute::BR1, &name)
            .await
            .map_err(RequestError::RequestFailed)
            .and_then(|summoner| summoner.map(Summoner).ok_or(RequestError::NotFound))
    }

    pub async fn matches(
        &self,
        client: &Client,
        range: std::ops::Range<u32>,
        queue: impl Into<Option<Queue>>,
    ) -> Result<impl Iterator<Item = game::Id>, RequestError> {
        client
            .0
            .match_v5()
            .get_match_ids_by_puuid(
                RegionalRoute::AMERICAS,
                &self.0.puuid,
                Some((range.end - range.start) as i32),
                None,
                queue.into().map(Queue::into),
                None,
                Some(range.start as i32),
                None,
            )
            .await
            .map_err(RequestError::RequestFailed)
            .map(|list| list.into_iter().filter_map(|s| s.try_into().ok()))
    }

    pub async fn leagues(
        &self,
        client: &Client,
    ) -> Result<impl Iterator<Item = League>, RequestError> {
        client
            .0
            .league_v4()
            .get_league_entries_for_summoner(PlatformRoute::BR1, &self.0.id)
            .await
            .map_err(RequestError::RequestFailed)
            .map(|leagues| leagues.into_iter().map(League))
    }
}

#[derive(Debug)]
pub struct League(riven::models::league_v4::LeagueEntry);
