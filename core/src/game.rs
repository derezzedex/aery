pub mod player;
use std::fmt;

pub use player::Player;

pub mod item;
pub mod rune;
pub use item::Item;

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

#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Event(pub riven::models::match_v5::EventsTimeLine);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Timeline(pub Vec<Event>);

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Game(pub riven::models::match_v5::Match);

impl Game {
    pub fn id(&self) -> Id {
        Id(self.0.metadata.match_id.clone())
    }

    pub fn queue(&self) -> Queue {
        self.0.info.queue_id.into()
    }

    pub fn created_at(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp_nanos(
            self.0.info.game_creation as i128 * 1_000_000,
        )
        .unwrap()
    }

    pub fn duration(&self) -> time::Duration {
        use time::ext::NumericalDuration;

        match self.0.info.game_end_timestamp {
            Some(_) => self.0.info.game_duration.seconds(),
            None => self.0.info.game_duration.milliseconds(),
        }
    }

    pub fn participants(&self) -> Vec<Player> {
        self.0.info.participants.iter().map(Player::from).collect()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Bottom,
    Jungle,
    Mid,
    Support,
    Top,
}

impl TryFrom<&String> for Role {
    type Error = ();

    fn try_from(role: &String) -> core::result::Result<Self, Self::Error> {
        match role.as_str() {
            "BOTTOM" => Ok(Role::Bottom),
            "JUNGLE" => Ok(Role::Jungle),
            "MIDDLE" => Ok(Role::Mid),
            "UTILITY" => Ok(Role::Support),
            "TOP" => Ok(Role::Top),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Queue::Custom => "Custom",
            Queue::Blind => "Blind Pick",
            Queue::Draft => "Draft Pick",
            Queue::RankedSolo => "Ranked Solo",
            Queue::RankedFlex => "Ranked Flex",
            Queue::Clash => "Clash",
            Queue::ARAM => "ARAM",
            Queue::BotIntro => "Bot (Introduction)",
            Queue::BotBeginner => "Bot (Beginner)",
            Queue::BotIntermediate => "Bot (Intermediate)",
            Queue::Other(_) => "Event",
            Queue::Unknown(_) => "Unknown",
        };

        write!(f, "{name}")
    }
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
