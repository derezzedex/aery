pub mod league;
pub use league::{Division, League, Tier};

use crate::account;
use crate::game;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub summoner: Summoner,
    pub leagues: Vec<League>,
    pub games: Vec<game::Game>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Summoner {
    pub riot_id: account::RiotId,
    pub raw: riven::models::summoner_v4::Summoner,
}

impl Summoner {
    pub fn name(&self) -> String {
        let name = self.riot_id.name.clone().unwrap_or_default();
        let tagline = self.riot_id.tagline.clone().unwrap_or_default();
        format!("{name}#{tagline}")
    }

    pub fn puuid(&self) -> &str {
        &self.raw.puuid
    }

    pub fn level(&self) -> u32 {
        self.raw.summoner_level as u32
    }

    pub fn icon_id(&self) -> i32 {
        self.raw.profile_icon_id
    }
}
