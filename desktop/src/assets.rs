use iced::widget::image::Handle;
use image::GenericImageView;

use std::collections::HashMap;
use std::fs;
use std::io::Read;

use crate::core;
use crate::core::game::rune;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DataFile {
    Champion,
    Item,
    ProfileIcon,
    RuneReforged,
    SummonerSpell,
}

impl TryFrom<String> for DataFile {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.split(".").next().unwrap();
        match value.to_ascii_lowercase().as_str() {
            "champion" => Ok(Self::Champion),
            "item" => Ok(Self::Item),
            "profileicon" => Ok(Self::ProfileIcon),
            "runesreforged" => Ok(Self::RuneReforged),
            "summoner" => Ok(Self::SummonerSpell),
            _ => Err("unknown data type"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Sprite {
    Champion(u8),
    Item(u8),
    SummonerSpell(u8),
    ProfileIcon(u8),
    RuneReforged(u8),
}

impl TryFrom<String> for Sprite {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.split(".").next().unwrap();
        let mut size = 0;
        let index: u8 = value
            .chars()
            .filter(|c| c.is_digit(10))
            .inspect(|_| size += 1)
            .collect::<String>()
            .parse()
            .map_err(|_| "index is not u8")?;
        let value = value[..value.len() - size].to_string();

        match value.to_ascii_lowercase().as_str() {
            "champion" => Ok(Self::Champion(index)),
            "item" => Ok(Self::Item(index)),
            "profileicon" => Ok(Self::ProfileIcon(index)),
            "runereforged" => Ok(Self::RuneReforged(index)),
            "spell" => Ok(Self::SummonerSpell(index)),
            _ => Err("unknown sprite type"),
        }
    }
}

pub type SpriteMap = HashMap<Sprite, image::DynamicImage>;

pub type DataMap = HashMap<DataFile, serde_json::Value>;

pub type RuneMap = HashMap<rune::Rune, String>;

pub type EmblemMap = HashMap<String, Handle>;

#[derive(Debug, Clone)]
pub struct Assets {
    pub sprites: SpriteMap,
    pub data: DataMap,
    pub runes: RuneMap,
    pub emblems: EmblemMap,
}

impl Assets {
    const SPRITE_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\img\\sprite");
    const DATA_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\data");
    const RUNES_PATH: &'static str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\data\\runesReforged.json"
    );
    const EMBLEMS_PATH: &'static str =
        concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\img\\emblems");

    pub async fn new() -> Assets {
        let timer = std::time::Instant::now();
        let mut sprites = HashMap::default();
        let img_path = fs::read_dir(Assets::SPRITE_PATH).unwrap();
        for sprite in img_path {
            let file = sprite.unwrap();
            let sprite = {
                let name = file.file_name().into_string().unwrap();
                name.try_into().unwrap()
            };
            let image = image::io::Reader::open(file.path())
                .unwrap()
                .decode()
                .unwrap();

            sprites.insert(sprite, image);
        }
        tracing::debug!("Loaded sprites in {:?}", timer.elapsed());

        let json_timer = std::time::Instant::now();
        let mut data = HashMap::default();
        let data_path = fs::read_dir(Assets::DATA_PATH).unwrap();
        for data_dir in data_path {
            let file = data_dir.unwrap();
            let sprite = {
                let name = file.file_name().into_string().unwrap();
                name.try_into().unwrap()
            };
            let mut bytes = Vec::new();
            fs::File::open(file.path())
                .unwrap()
                .read_to_end(&mut bytes)
                .unwrap();
            let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

            data.insert(sprite, value);
        }
        tracing::debug!("Loaded JSON data in {:?}", json_timer.elapsed());

        let runes_timer = std::time::Instant::now();
        let mut runes = HashMap::default();
        let value: serde_json::Value =
            serde_json::from_reader(fs::File::open(Assets::RUNES_PATH).unwrap()).unwrap();

        for value in value.as_array().unwrap() {
            let path = value["icon"]
                .as_str()
                .unwrap()
                .trim_start_matches("perk-images/");
            let id = value["id"].as_u64().unwrap();
            runes.insert(rune::Rune(id as usize), path.to_string());

            for slots in value["slots"].as_array().unwrap() {
                for rune in slots["runes"].as_array().unwrap() {
                    let path = rune["icon"]
                        .as_str()
                        .unwrap()
                        .trim_start_matches("perk-images/");
                    let lesser_id = rune["id"].as_u64().unwrap();
                    runes.insert(rune::Rune(lesser_id as usize), path.to_string());
                }
            }
        }
        tracing::debug!("Loaded rune data in {:?}", runes_timer.elapsed());

        let emblem_timer = std::time::Instant::now();
        let mut emblems = HashMap::default();
        let img_path = fs::read_dir(Assets::EMBLEMS_PATH).unwrap();
        for sprite in img_path {
            let file = sprite.unwrap();
            let sprite = {
                let name = file.file_name().into_string().unwrap();
                name.try_into().unwrap()
            };
            let image = iced::widget::image::Handle::from_path(file.path());

            emblems.insert(sprite, image);
        }
        tracing::debug!("Loaded emblems in {:?}", emblem_timer.elapsed());
        tracing::debug!("Total time: {:?}", timer.elapsed());

        Assets {
            sprites,
            data,
            runes,
            emblems,
        }
    }
}

// TODO: use champion id instead of name
pub fn load_champion_icon(assets: &Assets, champion: core::Champion) -> Handle {
    let icon_data = assets.data.get(&DataFile::Champion).unwrap();
    let name = champion.identifier().unwrap();
    let icon = &icon_data["data"][name]["image"];
    let sprite = Sprite::try_from(icon["sprite"].as_str().unwrap().to_string()).unwrap();
    let x = icon["x"].as_u64().unwrap() as u32;
    let y = icon["y"].as_u64().unwrap() as u32;
    let w = icon["w"].as_u64().unwrap() as u32;
    let h = icon["h"].as_u64().unwrap() as u32;
    let offset = 3;

    let icon_sprite = assets.sprites.get(&sprite).unwrap();
    let icon = icon_sprite.view(x + offset, y + offset, w - offset * 2, h - offset * 2);
    Handle::from_pixels(icon.width(), icon.height(), icon.to_image().into_vec())
}

pub fn load_summoner_spell_icon(assets: &Assets, summoner_spell: core::SummonerSpell) -> Handle {
    let icon_data = assets.data.get(&DataFile::SummonerSpell).unwrap();
    let spell = {
        let data = icon_data["data"].as_object().unwrap();
        data.iter()
            .find(|(_, data)| data["key"] == summoner_spell.id().to_string().as_str())
            .and_then(|(_, data)| data["id"].as_str())
            .unwrap()
    };
    let icon = &icon_data["data"][spell]["image"];
    let sprite = Sprite::try_from(icon["sprite"].as_str().unwrap().to_string()).unwrap();
    let x = icon["x"].as_u64().unwrap() as u32;
    let y = icon["y"].as_u64().unwrap() as u32;
    let w = icon["w"].as_u64().unwrap() as u32;
    let h = icon["h"].as_u64().unwrap() as u32;
    let offset = 0;

    let icon_sprite = assets.sprites.get(&sprite).unwrap();
    let icon = icon_sprite.view(x + offset, y + offset, w - offset * 2, h - offset * 2);
    Handle::from_pixels(icon.width(), icon.height(), icon.to_image().into_vec())
}

pub fn load_runes_icon(assets: &Assets, rune: rune::Rune) -> Handle {
    let rune_path = assets.runes.get(&rune).unwrap();
    let mut path = std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\runes\\"
    ));
    path.push(rune_path);

    Handle::from_path(path)
}

pub fn load_item_icon(assets: &Assets, item: core::Item) -> Handle {
    let icon_data = assets.data.get(&DataFile::Item).unwrap();
    let icon = &icon_data["data"][item.to_string()]["image"];
    let sprite = Sprite::try_from(icon["sprite"].as_str().unwrap().to_string()).unwrap();
    let x = icon["x"].as_u64().unwrap() as u32;
    let y = icon["y"].as_u64().unwrap() as u32;
    let w = icon["w"].as_u64().unwrap() as u32;
    let h = icon["h"].as_u64().unwrap() as u32;
    let offset = 0;

    let icon_sprite = assets.sprites.get(&sprite).unwrap();
    let icon = icon_sprite.view(x + offset, y + offset, w - offset * 2, h - offset * 2);
    Handle::from_pixels(icon.width(), icon.height(), icon.to_image().into_vec())
}
