pub mod client;

use std::str::FromStr;

pub use client::Client;

pub mod summoner;
use riven::consts::{PlatformRoute, RegionalRoute};
pub use summoner::Summoner;

pub mod game;
pub use game::Game;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Team(usize);

impl Team {
    pub const BLUE: Team = Team(100);
    pub const RED: Team = Team(200);
}

#[derive(Debug, Clone)]
pub enum Route {
    America,
    Asia,
    Europe,
    SouthAsia,
    Tournament,
}

impl From<RegionalRoute> for Route {
    fn from(route: RegionalRoute) -> Self {
        match route {
            RegionalRoute::AMERICAS => Route::America,
            RegionalRoute::ASIA => Route::Asia,
            RegionalRoute::EUROPE => Route::Europe,
            RegionalRoute::SEA => Route::SouthAsia,
            RegionalRoute::ESPORTS => Route::Tournament,
            #[allow(deprecated)]
            RegionalRoute::APAC => Route::SouthAsia,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region(PlatformRoute);

impl From<String> for Region {
    fn from(value: String) -> Self {
        Self(PlatformRoute::from_str(&value).unwrap())
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl Default for Region {
    fn default() -> Self {
        Region(PlatformRoute::BR1)
    }
}

impl Region {
    pub fn iter() -> Vec<Region> {
        use riven::consts::IntoEnumIterator;

        PlatformRoute::iter().map(Region).collect()
    }
}

impl From<PlatformRoute> for Region {
    fn from(route: PlatformRoute) -> Self {
        Region(route)
    }
}
