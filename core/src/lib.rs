pub mod client;

pub use client::Client;

pub mod summoner;
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
