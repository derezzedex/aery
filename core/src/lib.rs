use riven::consts::{PlatformRoute, RegionalRoute};

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
        queue: impl Into<Option<Queue>>,
    ) -> Result<impl Iterator<Item = MatchId>, SummonerRequestError> {
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
            .map_err(SummonerRequestError::RequestFailed)
            .map(|list| list.into_iter().map(MatchId))
    }

    pub async fn leagues(
        &self,
        client: &Client,
    ) -> Result<impl Iterator<Item = League>, SummonerRequestError> {
        client
            .0
            .league_v4()
            .get_league_entries_for_summoner(PlatformRoute::BR1, &self.0.id)
            .await
            .map_err(SummonerRequestError::RequestFailed)
            .map(|leagues| leagues.into_iter().map(League))
    }
}

#[derive(Debug)]
pub struct League(riven::models::league_v4::LeagueEntry);

#[derive(Debug)]
pub enum Queue {
    /// CUSTOM
    Custom,
    /// SUMMONERS_RIFT_5V5_BLIND_PICK
    Blind,
    /// SUMMONERS_RIFT_5V5_DRAFT_PICK
    Draft,
    /// SUMMONERS_RIFT_5V5_RANKED_SOLO
    RankedSolo,
    /// SUMMONERS_RIFT_5V5_RANKED_FLEX
    RankedFlex,
    /// SUMMONERS_RIFT_CLASH
    Clash,
    /// HOWLING_ABYSS_5V5_ARAM,
    ARAM,

    BotIntro,
    BotBeginner,
    BotIntermediate,

    Other(u16),
    Unknown(u16),
}

impl From<Queue> for riven::consts::Queue {
    fn from(queue: Queue) -> riven::consts::Queue {
        match queue {
            Queue::Custom => riven::consts::Queue::CUSTOM,
            Queue::Blind => riven::consts::Queue::SUMMONERS_RIFT_5V5_BLIND_PICK,
            Queue::Draft => riven::consts::Queue::SUMMONERS_RIFT_5V5_DRAFT_PICK,
            Queue::RankedSolo => riven::consts::Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO,
            Queue::RankedFlex => riven::consts::Queue::SUMMONERS_RIFT_5V5_RANKED_FLEX,
            Queue::Clash => riven::consts::Queue::SUMMONERS_RIFT_CLASH,
            Queue::ARAM => riven::consts::Queue::HOWLING_ABYSS_5V5_ARAM,
            Queue::BotIntro => riven::consts::Queue::SUMMONERS_RIFT_CO_OP_VS_AI_INTRO_BOT,
            Queue::BotBeginner => riven::consts::Queue::SUMMONERS_RIFT_CO_OP_VS_AI_BEGINNER_BOT,
            Queue::BotIntermediate => {
                riven::consts::Queue::SUMMONERS_RIFT_CO_OP_VS_AI_INTERMEDIATE_BOT
            }
            Queue::Other(id) => riven::consts::Queue::from(id),
            Queue::Unknown(id) => riven::consts::Queue::from(id),
        }
    }
}

impl From<riven::consts::Queue> for Queue {
    fn from(queue: riven::consts::Queue) -> Queue {
        use riven::consts::Queue as ApiQueue;

        if !queue.is_known() {
            tracing::debug!("Unknown queue id encountered ({}).", queue.0);
            return Queue::Unknown(queue.0);
        }

        match queue {
            ApiQueue::CUSTOM => Queue::Custom,
            ApiQueue::SUMMONERS_RIFT_5V5_BLIND_PICK => Queue::Blind,
            ApiQueue::SUMMONERS_RIFT_5V5_DRAFT_PICK => Queue::Draft,
            ApiQueue::SUMMONERS_RIFT_5V5_RANKED_SOLO => Queue::RankedSolo,
            ApiQueue::SUMMONERS_RIFT_5V5_RANKED_FLEX => Queue::RankedFlex,
            ApiQueue::SUMMONERS_RIFT_CLASH => Queue::Clash,
            ApiQueue::HOWLING_ABYSS_5V5_ARAM => Queue::ARAM,
            ApiQueue::SUMMONERS_RIFT_CO_OP_VS_AI_INTRO_BOT => Queue::BotIntro,
            ApiQueue::SUMMONERS_RIFT_CO_OP_VS_AI_BEGINNER_BOT => Queue::BotBeginner,
            ApiQueue::SUMMONERS_RIFT_CO_OP_VS_AI_INTERMEDIATE_BOT => Queue::BotIntermediate,
            ApiQueue(id) => Queue::Other(id),
        }
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

#[derive(Debug, Clone)]
pub struct MatchEvent(riven::models::match_v5::MatchTimelineInfoFrameEvent);

impl MatchEvent {
    pub async fn from_id(client: &Client, id: MatchId) -> Result<Vec<Self>, MatchRequestError> {
        client
            .0
            .match_v5()
            .get_timeline(RegionalRoute::AMERICAS, &id.0)
            .await
            .map_err(MatchRequestError::RequestFailed)
            .and_then(|timeline| {
                timeline
                    .map(|tl| {
                        tl.info
                            .frames
                            .into_iter()
                            .flat_map(|frame| frame.events.into_iter())
                            .map(MatchEvent)
                            .collect::<Vec<_>>()
                    })
                    .ok_or(MatchRequestError::NotFound)
            })
    }
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

    pub async fn events(&self, client: &Client) -> Result<Vec<MatchEvent>, MatchRequestError> {
        client
            .0
            .match_v5()
            .get_timeline(RegionalRoute::AMERICAS, &self.0.metadata.match_id)
            .await
            .map_err(MatchRequestError::RequestFailed)
            .and_then(|timeline| {
                timeline
                    .map(|tl| {
                        tl.info
                            .frames
                            .into_iter()
                            .flat_map(|frame| frame.events.into_iter())
                            .map(MatchEvent)
                            .collect::<Vec<_>>()
                    })
                    .ok_or(MatchRequestError::NotFound)
            })
    }

    pub fn participant(&self, puuid: &str) -> Option<&riven::models::match_v5::Participant> {
        self.0
            .info
            .participants
            .iter()
            .find(|participant| participant.puuid == puuid)
    }
}
