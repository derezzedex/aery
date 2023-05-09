use riven::consts::{PlatformRoute, Queue, RegionalRoute};

pub struct Client(riven::RiotApi);

impl Client {
    pub fn new(key: String) -> Self {
        Client(riven::RiotApi::new(key))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SummonerRequestError {
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

    pub async fn from_name(client: &Client, name: &str) -> Result<Self, SummonerRequestError> {
        client
            .0
            .summoner_v4()
            .get_by_summoner_name(PlatformRoute::BR1, &name)
            .await
            .map_err(SummonerRequestError::RequestFailed)
            .and_then(|summoner| summoner.map(Summoner).ok_or(SummonerRequestError::NotFound))
    }

    pub async fn matches(
        &self,
        client: &Client,
        range: std::ops::Range<u32>,
    ) -> Result<impl Iterator<Item = MatchId>, SummonerRequestError> {
        client
            .0
            .match_v5()
            .get_match_ids_by_puuid(
                RegionalRoute::AMERICAS,
                &self.0.puuid,
                Some((range.end - range.start) as i32),
                None,
                Some(Queue::SUMMONERS_RIFT_5V5_RANKED_FLEX),
                None,
                Some(range.start as i32),
                None,
            )
            .await
            .map_err(SummonerRequestError::RequestFailed)
            .map(|list| list.into_iter().map(MatchId))
    }
}

#[derive(Debug)]
pub struct MatchId(String);

#[derive(Debug, thiserror::Error)]
pub enum MatchRequestError {
    #[error("not found")]
    NotFound,
    #[error(transparent)]
    RequestFailed(#[from] riven::RiotApiError),
}

#[derive(Debug)]
pub struct Match(riven::models::match_v5::Match);

impl Match {
    pub fn id(&self) -> MatchId {
        MatchId(self.0.metadata.match_id.clone())
    }

    pub async fn from_id(client: &Client, id: MatchId) -> Result<Self, MatchRequestError> {
        client
            .0
            .match_v5()
            .get_match(RegionalRoute::AMERICAS, &id.0)
            .await
            .map_err(MatchRequestError::RequestFailed)
            .and_then(|game| game.map(Match).ok_or(MatchRequestError::NotFound))
    }

    pub fn participant(&self, puuid: &str) -> Option<&riven::models::match_v5::Participant> {
        self.0
            .info
            .participants
            .iter()
            .find(|participant| participant.puuid == puuid)
    }
}
