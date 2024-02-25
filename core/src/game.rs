pub mod player;
pub use player::Player;

use crate::{Client, Duration, Queue, Time};

use riven::consts::RegionalRoute;

#[derive(Debug)]
pub struct Id(String);

impl TryFrom<String> for Id {
    type Error = ();

    fn try_from(value: String) -> core::result::Result<Self, Self::Error> {
        // TODO: verify this value

        Ok(Id(value))
    }
}

impl AsRef<String> for Id {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("not found")]
    NotFound,
    #[error(transparent)]
    RequestFailed(#[from] riven::RiotApiError),
}

#[derive(Debug, Clone)]
pub struct Event(riven::models::match_v5::MatchTimelineInfoFrameEvent);

impl Event {
    pub async fn from_id(client: &Client, id: Id) -> core::result::Result<Vec<Self>, RequestError> {
        client
            .as_ref()
            .match_v5()
            .get_timeline(RegionalRoute::AMERICAS, &id.0)
            .await
            .map_err(RequestError::RequestFailed)
            .and_then(|timeline| {
                timeline
                    .map(|tl| {
                        tl.info
                            .frames
                            .into_iter()
                            .flat_map(|frame| frame.events.into_iter())
                            .map(Event)
                            .collect::<Vec<_>>()
                    })
                    .ok_or(RequestError::NotFound)
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Result {
    Defeat,
    Remake,
    Surrender,
    Victory,
}

impl From<bool> for Result {
    fn from(won: bool) -> Self {
        match won {
            true => Self::Victory,
            false => Self::Defeat,
        }
    }
}

impl Result {
    pub fn won(&self) -> bool {
        *self == Self::Victory
    }

    pub fn lost(&self) -> bool {
        *self == Self::Defeat || *self == Self::Surrender
    }
}

#[derive(Debug, Clone)]
pub struct Game(riven::models::match_v5::Match);

impl Game {
    pub fn id(&self) -> Id {
        Id(self.0.metadata.match_id.clone())
    }

    pub fn queue(&self) -> Queue {
        self.0.info.queue_id.into()
    }

    pub fn created_at(&self) -> Time {
        Time(
            time::OffsetDateTime::from_unix_timestamp_nanos(
                self.0.info.game_creation as i128 * 1_000_000,
            )
            .unwrap(),
        )
    }

    pub fn duration(&self) -> Duration {
        use time::ext::NumericalDuration;

        match self.0.info.game_end_timestamp {
            Some(_) => Duration(self.0.info.game_duration.seconds()),
            None => Duration(self.0.info.game_duration.milliseconds()),
        }
    }

    pub fn participants(&self) -> Vec<Player> {
        self.0.info.participants.iter().map(Player::from).collect()
    }

    pub async fn from_id(client: &Client, id: Id) -> core::result::Result<Self, RequestError> {
        client
            .as_ref()
            .match_v5()
            .get_match(RegionalRoute::AMERICAS, &id.0)
            .await
            .map_err(RequestError::RequestFailed)
            .and_then(|game| game.map(Game).ok_or(RequestError::NotFound))
    }

    pub async fn events(&self, client: &Client) -> core::result::Result<Vec<Event>, RequestError> {
        client
            .as_ref()
            .match_v5()
            .get_timeline(RegionalRoute::AMERICAS, &self.0.metadata.match_id)
            .await
            .map_err(RequestError::RequestFailed)
            .and_then(|timeline| {
                timeline
                    .map(|tl| {
                        tl.info
                            .frames
                            .into_iter()
                            .flat_map(|frame| frame.events.into_iter())
                            .map(Event)
                            .collect::<Vec<_>>()
                    })
                    .ok_or(RequestError::NotFound)
            })
    }

    pub fn participant(&self, puuid: &str) -> Option<Player> {
        self.0
            .info
            .participants
            .iter()
            .find(|participant| participant.puuid == puuid)
            .map(Player::from)
    }
}
