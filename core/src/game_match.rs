use crate::{Client, Duration, Participant, Queue};

use riven::consts::RegionalRoute;

#[derive(Debug)]
pub struct Id(String);

impl TryFrom<String> for Id {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
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
    pub async fn from_id(client: &Client, id: Id) -> Result<Vec<Self>, RequestError> {
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

#[derive(Debug, Clone)]
pub struct GameMatch(riven::models::match_v5::Match);

impl GameMatch {
    pub fn id(&self) -> Id {
        Id(self.0.metadata.match_id.clone())
    }

    pub fn queue(&self) -> Queue {
        self.0.info.queue_id.into()
    }

    pub fn duration(&self) -> Duration {
        use time::ext::NumericalDuration;

        match self.0.info.game_end_timestamp {
            Some(_) => Duration(self.0.info.game_duration.seconds()),
            None => Duration(self.0.info.game_duration.milliseconds()),
        }
    }

    pub fn participants(&self) -> Vec<Participant> {
        self.0
            .info
            .participants
            .iter()
            .map(Participant::from)
            .collect()
    }

    pub async fn from_id(client: &Client, id: Id) -> Result<Self, RequestError> {
        client
            .as_ref()
            .match_v5()
            .get_match(RegionalRoute::AMERICAS, &id.0)
            .await
            .map_err(RequestError::RequestFailed)
            .and_then(|game| game.map(GameMatch).ok_or(RequestError::NotFound))
    }

    pub async fn events(&self, client: &Client) -> Result<Vec<Event>, RequestError> {
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

    pub fn participant(&self, puuid: &str) -> Option<&riven::models::match_v5::Participant> {
        self.0
            .info
            .participants
            .iter()
            .find(|participant| participant.puuid == puuid)
    }
}
