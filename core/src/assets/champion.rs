use crate::assets;
use std::collections::HashMap;

#[derive(bitcode::Encode, bitcode::Decode, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Id(pub u16);

impl Id {
    pub fn from_key(key: &String) -> Self {
        Self(key.parse().unwrap())
    }
}

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Assets {
    pub icon: assets::Image,
}

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct AssetMap(pub HashMap<Id, Assets>);
