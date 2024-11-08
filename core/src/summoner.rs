pub mod league;
pub use league::{Division, League, Tier};

use crate::game;
use crate::Client;
use crate::Region;
use riven::consts::RegionalRoute;

#[allow(dead_code)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Summoner {
    name: String,
    summoner: riven::models::summoner_v4::Summoner,
}

impl Summoner {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn puuid(&self) -> &str {
        &self.summoner.puuid
    }

    pub fn level(&self) -> u32 {
        self.summoner.summoner_level as u32
    }

    pub fn icon_id(&self) -> i32 {
        self.summoner.profile_icon_id
    }

    pub async fn from_name(
        client: Client,
        name: String,
        region: Region,
    ) -> Result<Self, RequestError> {
        let mut account_id = name.split("#");

        let (game_name, tag_line) = (
            account_id.next().ok_or(RequestError::NotFound)?,
            account_id.next().ok_or(RequestError::NotFound)?,
        );

        tracing::info!("Requesting account: {game_name}#{tag_line}");

        let account = client
            .as_ref()
            .account_v1()
            .get_by_riot_id(RegionalRoute::AMERICAS, game_name, tag_line)
            .await
            .map_err(|error| RequestError::RequestFailed(InternalApiError(error.to_string())))?
            .ok_or(RequestError::NotFound)?;

        client
            .as_ref()
            .summoner_v4()
            .get_by_puuid(region.0, &account.puuid)
            .await
            .map_err(|error| RequestError::RequestFailed(InternalApiError(error.to_string())))
            .map(|summoner| Summoner { name, summoner })
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
                &self.summoner.puuid,
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
        region: Region,
    ) -> Result<impl Iterator<Item = League>, RequestError> {
        client
            .as_ref()
            .league_v4()
            .get_league_entries_for_summoner(region.0, &self.summoner.id)
            .await
            .map_err(|error| RequestError::RequestFailed(InternalApiError(error.to_string())))
            .map(|leagues| leagues.into_iter().map(League))
    }
}
