pub mod league;
pub use league::{Division, League, Tier};

use crate::account;
use crate::game;
use crate::Client;
use crate::Region;
use crate::RequestError;

use riven::consts::RegionalRoute;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub summoner: Summoner,
    pub leagues: Vec<League>,
    pub games: Vec<game::Game>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Summoner {
    pub riot_id: account::RiotId,
    pub raw: riven::models::summoner_v4::Summoner,
}

impl Summoner {
    pub fn name(&self) -> String {
        let name = self.riot_id.name.clone().unwrap_or_default();
        let tagline = self.riot_id.tagline.clone().unwrap_or_default();
        format!("{name}#{tagline}")
    }

    pub fn puuid(&self) -> &str {
        &self.raw.puuid
    }

    pub fn level(&self) -> u32 {
        self.raw.summoner_level as u32
    }

    pub fn icon_id(&self) -> i32 {
        self.raw.profile_icon_id
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

        let riot_id = account::RiotId::new(game_name, tag_line);

        let account = client
            .as_ref()
            .account_v1()
            .get_by_riot_id(RegionalRoute::AMERICAS, game_name, tag_line)
            .await
            .map_err(RequestError::internal)?
            .ok_or(RequestError::NotFound)?;

        client
            .as_ref()
            .summoner_v4()
            .get_by_puuid(region.0, &account.puuid)
            .await
            .map_err(RequestError::internal)
            .map(|summoner| Summoner {
                riot_id,
                raw: summoner,
            })
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
                &self.raw.puuid,
                Some((range.end - range.start) as i32),
                None,
                queue.into().map(game::Queue::into),
                None,
                Some(range.start as i32),
                None,
            )
            .await
            .map_err(RequestError::internal)
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
            .get_league_entries_for_summoner(region.0, &self.raw.id)
            .await
            .map_err(RequestError::internal)
            .map(|leagues| leagues.into_iter().map(League))
    }
}

pub async fn matches(
    puuid: &str,
    client: &Client,
    range: std::ops::Range<u32>,
    queue: impl Into<Option<game::Queue>>,
) -> Result<impl Iterator<Item = game::Id>, RequestError> {
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
        .map_err(RequestError::internal)
        .map(|list| list.into_iter().filter_map(|s| s.try_into().ok()))
}
