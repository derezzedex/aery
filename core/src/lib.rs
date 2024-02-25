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
