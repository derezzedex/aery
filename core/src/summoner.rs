pub mod league;
pub use league::{Division, League, Tier};

use crate::game;
use crate::Account;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub icon: Vec<u8>,
    pub summoner: Summoner,
    pub leagues: Vec<League>,
    pub games: Vec<game::Game>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Summoner {
    pub account: Account,
    pub level: i64,
    pub icon_id: i32,
    pub last_modified: i64,
}

impl Summoner {
    pub fn name(&self) -> String {
        let name = self.account.riot_id.name.clone().unwrap_or_default();
        let tagline = self.account.riot_id.tagline.clone().unwrap_or_default();
        format!("{name}#{tagline}")
    }

    pub fn puuid(&self) -> &str {
        self.account.puuid.as_ref()
    }
}
