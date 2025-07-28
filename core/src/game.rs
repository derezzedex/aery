pub mod player;
pub use player::Player;

pub mod item;
pub mod rune;
pub use item::Item;

use riven::models::match_v5;
use std::collections::{HashMap, hash_map};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, bitcode::Encode, bitcode::Decode)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, bitcode::Encode, bitcode::Decode)]
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

#[derive(Debug, Default, Clone, bitcode::Encode, bitcode::Decode)]
pub struct Map(HashMap<Id, Game>);

impl Map {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn encode(&self) -> Vec<u8> {
        let bytes = bitcode::encode(self);
        lz4_flex::compress_prepend_size(&bytes)
    }

    pub fn decode(bytes: &[u8]) -> Self {
        let decompressed = lz4_flex::decompress_size_prepended(bytes).unwrap();
        bitcode::decode(&decompressed).unwrap()
    }

    pub fn iter(&self) -> hash_map::Iter<'_, Id, Game> {
        self.0.iter()
    }
}

impl From<HashMap<Id, Game>> for Map {
    fn from(games: HashMap<Id, Game>) -> Self {
        Self(games)
    }
}

impl FromIterator<(Id, Game)> for Map {
    fn from_iter<T: IntoIterator<Item = (Id, Game)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl Extend<(Id, Game)> for Map {
    fn extend<T: IntoIterator<Item = (Id, Game)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct Game {
    pub id: Id,
    pub queue: Queue,
    pub created_at: i64,
    pub duration: i64,
    pub players: Vec<Player>,
}

impl Game {
    pub fn created_at_time(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp_nanos(self.created_at as i128 * 1_000_000)
            .unwrap()
    }

    pub fn duration_time(&self) -> time::Duration {
        time::Duration::seconds(self.duration)
    }

    pub fn player(&self, puuid: &str) -> Option<&Player> {
        self.players
            .iter()
            .find(|participant| participant.puuid == puuid)
    }

    #[cfg(feature = "dummy")]
    pub fn dummy(riot_id: crate::account::RiotId) -> Self {
        use crate::{Champion, Team, account};

        let players = vec![
            Player::dummy(
                riot_id,
                Team::RED,
                Role::Top,
                Champion(799),
                Result::Victory,
            ),
            Player::dummy(
                account::RiotId::new("Brambleback", "red"),
                Team::RED,
                Role::Jungle,
                Champion(233),
                Result::Victory,
            ),
            Player::dummy(
                account::RiotId::new("Gromp", "red"),
                Team::RED,
                Role::Mid,
                Champion(800),
                Result::Victory,
            ),
            Player::dummy(
                account::RiotId::new("Murk Wolf", "red"),
                Team::RED,
                Role::Bottom,
                Champion(901),
                Result::Victory,
            ),
            Player::dummy(
                account::RiotId::new("Rift Scuttler", "red"),
                Team::RED,
                Role::Support,
                Champion(902),
                Result::Victory,
            ),
            Player::dummy(
                account::RiotId::new("Sentinel", "blue"),
                Team::BLUE,
                Role::Top,
                Champion(14),
                Result::Defeat,
            ),
            Player::dummy(
                account::RiotId::new("Krug", "blue"),
                Team::BLUE,
                Role::Jungle,
                Champion(9),
                Result::Defeat,
            ),
            Player::dummy(
                account::RiotId::new("Raptor", "blue"),
                Team::BLUE,
                Role::Mid,
                Champion(1),
                Result::Defeat,
            ),
            Player::dummy(
                account::RiotId::new("Voidmite", "blue"),
                Team::BLUE,
                Role::Bottom,
                Champion(22),
                Result::Defeat,
            ),
            Player::dummy(
                account::RiotId::new("Rift Herald", "blue"),
                Team::BLUE,
                Role::Support,
                Champion(12),
                Result::Defeat,
            ),
        ];

        Game {
            id: Id(String::from("foo")),
            queue: Queue::RankedSolo,
            created_at: 1751830754821,
            duration: 2205,
            players,
        }
    }
}

impl From<match_v5::Match> for Game {
    fn from(game: match_v5::Match) -> Self {
        let id = Id(game.metadata.match_id.clone());
        let queue = Queue::from(game.info.queue_id);
        let duration = if game.info.game_end_timestamp.is_some() {
            time::Duration::seconds(game.info.game_duration).whole_seconds()
        } else {
            time::Duration::milliseconds(game.info.game_duration).whole_seconds()
        };
        let players = game.info.participants.iter().map(Player::from).collect();

        Self {
            id,
            queue,
            created_at: game.info.game_creation,
            duration,
            players,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bitcode::Encode, bitcode::Decode)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bitcode::Encode, bitcode::Decode)]
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
