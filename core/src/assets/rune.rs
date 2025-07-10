use crate::assets;
use std::collections::HashMap;

#[derive(bitcode::Encode, bitcode::Decode, PartialEq, Eq, Hash)]
pub struct Id(pub u16);

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Assets {
    pub icon: assets::Image,
}

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct AssetMap(pub HashMap<Id, Assets>);
