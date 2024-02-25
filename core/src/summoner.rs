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

#[derive(Debug, Clone)]
pub struct League(riven::models::league_v4::LeagueEntry);

impl League {
    pub fn queue_kind(&self) -> game::Queue {
        use riven::consts::QueueType;

        match self.0.queue_type {
            QueueType::RANKED_SOLO_5x5 => game::Queue::RankedSolo,
            QueueType::RANKED_FLEX_SR => game::Queue::RankedFlex,
            _ => game::Queue::Unknown(0),
        }
    }

    pub fn tier(&self) -> Option<crate::Tier> {
        let points = self.points() as u16;
        let division = self.division();

        self.0
            .tier
            .filter(|&t| t != riven::consts::Tier::UNRANKED)
            .map(|tier| match tier {
                riven::consts::Tier::UNRANKED => unreachable!(),
                riven::consts::Tier::IRON => crate::Tier::Iron(division.unwrap()),
                riven::consts::Tier::BRONZE => crate::Tier::Bronze(division.unwrap()),
                riven::consts::Tier::SILVER => crate::Tier::Silver(division.unwrap()),
                riven::consts::Tier::GOLD => crate::Tier::Gold(division.unwrap()),
                riven::consts::Tier::PLATINUM => crate::Tier::Platinum(division.unwrap()),
                riven::consts::Tier::EMERALD => crate::Tier::Emerald(division.unwrap()),
                riven::consts::Tier::DIAMOND => crate::Tier::Diamond(division.unwrap()),
                riven::consts::Tier::MASTER => crate::Tier::Master(points),
                riven::consts::Tier::GRANDMASTER => crate::Tier::Grandmaster(points),
                riven::consts::Tier::CHALLENGER => crate::Tier::Challenger(points),
            })
    }

    #[allow(deprecated)]
    pub fn division(&self) -> Option<crate::Division> {
        self.0
            .rank
            .filter(|&d| d != riven::consts::Division::V)
            .map(|division| match division {
                riven::consts::Division::I => crate::Division::One(self.0.league_points as u8),
                riven::consts::Division::II => crate::Division::Two(self.0.league_points as u8),
                riven::consts::Division::III => crate::Division::Three(self.0.league_points as u8),
                riven::consts::Division::IV => crate::Division::Four(self.0.league_points as u8),
                _ => unreachable!(),
            })
    }

    pub fn points(&self) -> u32 {
        self.0.league_points as u32
    }
    pub fn wins(&self) -> u32 {
        self.0.wins as u32
    }
    pub fn losses(&self) -> u32 {
        self.0.losses as u32
    }
}
