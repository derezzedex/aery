use crate::assets;
use std::collections::HashMap;

#[derive(bitcode::Encode, bitcode::Decode, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Id(pub u8);

impl Id {
    pub const ALL: &[Id] = &[
        Id(0),
        Id(1),
        Id(2),
        Id(3),
        Id(4),
        Id(5),
        Id(6),
        Id(7),
        Id(8),
        Id(9),
    ];

    pub fn from_key(key: &String) -> Self {
        let id = match key.as_str() {
            "iron" => 0,
            "bronze" => 1,
            "silver" => 2,
            "gold" => 3,
            "platinum" => 4,
            "emerald" => 5,
            "diamond" => 6,
            "master" => 7,
            "grandmaster" => 8,
            "challenger" => 9,
            _ => 0,
        };

        Self(id)
    }

    pub fn into_key(&self) -> &'static str {
        match self.0 {
            0 => "iron",
            1 => "bronze",
            2 => "silver",
            3 => "gold",
            4 => "platinum",
            5 => "emerald",
            6 => "diamond",
            7 => "master",
            8 => "grandmaster",
            9 => "challenger",
            _ => unreachable!(),
        }
    }
}

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Assets {
    pub icon: assets::Image,
}

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct AssetMap(pub HashMap<Id, Assets>);
