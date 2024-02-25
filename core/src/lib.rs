pub mod client;
use std::fmt;

pub use client::Client;

pub mod summoner;
pub use summoner::Summoner;

pub mod game;
pub use game::Game;

#[derive(Debug, Copy, Clone)]
pub enum Tier {
    Iron(Division),
    Bronze(Division),
    Silver(Division),
    Gold(Division),
    Platinum(Division),
    Emerald(Division),
    Diamond(Division),
    Master(u16),
    Grandmaster(u16),
    Challenger(u16),
}

impl Tier {
    pub fn points(&self) -> u16 {
        match self {
            Tier::Challenger(points) | Tier::Grandmaster(points) | Tier::Master(points) => *points,
            Tier::Iron(division)
            | Tier::Bronze(division)
            | Tier::Silver(division)
            | Tier::Gold(division)
            | Tier::Platinum(division)
            | Tier::Emerald(division)
            | Tier::Diamond(division) => match division {
                Division::One(points)
                | Division::Two(points)
                | Division::Three(points)
                | Division::Four(points) => *points as u16,
            },
        }
    }

    pub fn division(&self) -> String {
        match self {
            Tier::Iron(division)
            | Tier::Bronze(division)
            | Tier::Silver(division)
            | Tier::Gold(division)
            | Tier::Platinum(division)
            | Tier::Emerald(division)
            | Tier::Diamond(division) => division.to_string(),
            Tier::Master(points) | Tier::Grandmaster(points) | Tier::Challenger(points) => {
                points.to_string()
            }
        }
    }
}

impl ToString for Tier {
    fn to_string(&self) -> String {
        match self {
            Tier::Iron(_) => "Iron",
            Tier::Bronze(_) => "Bronze",
            Tier::Silver(_) => "Silver",
            Tier::Gold(_) => "Gold",
            Tier::Platinum(_) => "Platinum",
            Tier::Emerald(_) => "Emerald",
            Tier::Diamond(_) => "Diamond",
            Tier::Master(_) => "Master",
            Tier::Grandmaster(_) => "Grandmaster",
            Tier::Challenger(_) => "Challenger",
        }
        .to_string()
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Division {
    One(u8),
    Two(u8),
    Three(u8),
    Four(u8),
}

impl ToString for Division {
    fn to_string(&self) -> String {
        match self {
            Division::One(_) => "1",
            Division::Two(_) => "2",
            Division::Three(_) => "3",
            Division::Four(_) => "4",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Duration(pub time::Duration);

impl ToString for Duration {
    fn to_string(&self) -> String {
        let minutes = self.0.whole_minutes().to_string();
        let seconds = self.0.whole_seconds().to_string();

        format!("{minutes:.2}m {seconds:.2}s")
    }
}

#[derive(Debug, Clone)]
pub struct Time(pub time::OffsetDateTime);

impl AsRef<time::OffsetDateTime> for Time {
    fn as_ref(&self) -> &time::OffsetDateTime {
        &self.0
    }
}

impl ToString for Time {
    fn to_string(&self) -> String {
        let now = time::OffsetDateTime::now_utc();
        let duration = now - self.0;
        let seconds = duration.whole_seconds();
        let minutes = seconds / 60;
        let hours = minutes / 60;
        let days = hours / 24;
        let weeks = days / 7;
        let months = days / 30;
        let years = days / 365;

        if seconds < 60 {
            String::from("few seconds ago")
        } else if minutes < 60 {
            format!(
                "{} minute{} ago",
                minutes,
                if minutes == 1 { "" } else { "s" }
            )
        } else if hours < 24 {
            format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
        } else if days < 7 {
            if days == 1 {
                String::from("yesterday")
            } else {
                format!("{} days ago", days)
            }
        } else if weeks < 4 {
            if weeks == 1 {
                String::from("last week")
            } else {
                format!("{} weeks ago", weeks)
            }
        } else if months < 12 {
            if months == 1 {
                String::from("last month")
            } else {
                format!("{} months ago", months)
            }
        } else {
            if years == 1 {
                return String::from("last year");
            } else {
                format!("{} years ago", years)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Bottom,
    Jungle,
    Mid,
    Support,
    Top,
    Unknown,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Bottom => "Bottom",
            Role::Jungle => "Jungle",
            Role::Mid => "Mid",
            Role::Support => "Support",
            Role::Top => "Top",
            Role::Unknown => "Unknown",
        }
        .to_string()
    }
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_str() {
            "BOTTOM" => Role::Bottom,
            "JUNGLE" => Role::Jungle,
            "MIDDLE" => Role::Mid,
            "UTILITY" => Role::Support,
            "TOP" => Role::Top,
            _ => Role::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Item(u32);

impl Item {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<i32> for Item {
    type Error = ();

    fn try_from(value: i32) -> Result<Item, Self::Error> {
        if value <= 0 {
            Err(())
        } else {
            Ok(Item(value as u32))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Trinket(pub u32);

impl Into<Item> for Trinket {
    fn into(self) -> Item {
        Item(self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Inventory(pub [Option<Item>; 6]);

impl IntoIterator for Inventory {
    type Item = Option<Item>;
    type IntoIter = std::array::IntoIter<Self::Item, 6>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Champion(u32);

impl Champion {
    pub fn new(id: u32) -> Self {
        //TODO: verify id
        Self(id)
    }

    pub fn identifier(&self) -> Option<&str> {
        // NOTE: Pretty sure this is a `riven` bug,
        // checking https://github.com/RiotGames/developer-relations/issues/7
        // shows that the `champion.json` uses `Fiddlesticks`!
        match self.0 {
            9 => Some("Fiddlesticks"),
            _ => riven::consts::Champion(self.0 as i16).identifier(),
        }
    }
}

impl From<riven::consts::Champion> for Champion {
    fn from(value: riven::consts::Champion) -> Self {
        Champion(value.0 as u32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SummonerSpell(u32);

impl SummonerSpell {
    pub fn new(id: u32) -> Self {
        //TODO: verify id
        Self(id)
    }
    pub fn id(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SummonerSpells(pub [SummonerSpell; 2]);

impl SummonerSpells {
    pub fn first(&self) -> SummonerSpell {
        self.0[0]
    }

    pub fn second(&self) -> SummonerSpell {
        self.0[1]
    }
}

impl From<[SummonerSpell; 2]> for SummonerSpells {
    fn from(spells: [SummonerSpell; 2]) -> Self {
        Self(spells)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Team(usize);

impl Team {
    pub const BLUE: Team = Team(100);
    pub const RED: Team = Team(200);
}
