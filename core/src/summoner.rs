pub mod league;
pub use league::{Division, League, Tier};

use crate::assets;
use crate::{Account, Game};

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct Data {
    pub icon: assets::Image,
    pub summoner: Summoner,
    pub leagues: Vec<League>,
    pub games: Vec<Game>,
}

impl Data {
    #[cfg(feature = "dummy")]
    pub fn dummy(name: impl ToString, tagline: impl ToString) -> Self {
        use crate::account;
        use bytes::Bytes;
        let bytes = Bytes::from_static(include_bytes!("../../assets/dummy/icon.jpg"));

        let dummy = Account::dummy(account::RiotId::new(name, tagline));

        Self {
            games: vec![Game::dummy(dummy.riot_id.clone()); 10],
            icon: assets::Image::from(bytes),
            summoner: Summoner {
                account: dummy,
                level: 404,
                icon_id: 3505,
                last_modified: 1751832991000,
            },
            leagues: vec![
                League::dummy(league::Kind::SummonersRift(league::SummonersRift::Solo)),
                League::dummy(league::Kind::SummonersRift(league::SummonersRift::Flex)),
            ],
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let bytes = bitcode::encode(self);
        lz4_flex::compress_prepend_size(&bytes)
    }

    pub fn decode(bytes: &[u8]) -> Self {
        let decompressed = lz4_flex::decompress_size_prepended(bytes).unwrap();
        bitcode::decode(&decompressed).unwrap()
    }
}

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
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
