use iced::font;
use iced::widget::image::Handle;
use iced::Task;
use image::GenericImageView;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::time::Instant;
use std::{ffi, fs, io};

use crate::core::game;
use crate::core::game::rune;
use crate::theme;
use crate::{core, Message};

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
        let value = value.split('.').next().unwrap();
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
    SummonerSpell(u16),
    ProfileIcon(u8),
    RuneReforged(u8),
}

impl TryFrom<String> for Sprite {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.split('.').next().unwrap();
        let mut size = 0;
        let index: u16 = value
            .chars()
            .filter(|c| c.is_ascii_digit())
            .inspect(|_| size += 1)
            .collect::<String>()
            .parse()
            .map_err(|_| "index is not u8")?;
        let value = value[..value.len() - size].to_string();

        match value.to_ascii_lowercase().as_str() {
            "champion" => Ok(Self::Champion(index as u8)),
            "item" => Ok(Self::Item(index as u8)),
            "profileicon" => Ok(Self::ProfileIcon(index as u8)),
            "runereforged" => Ok(Self::RuneReforged(index as u8)),
            "spell" => Ok(Self::SummonerSpell(index)),
            _ => Err("unknown sprite type"),
        }
    }
}

pub type SpriteMap = HashMap<Sprite, image::DynamicImage>;

pub type DataMap = HashMap<DataFile, serde_json::Value>;

pub type RuneMap = HashMap<rune::Rune, String>;

pub type EmblemMap = HashMap<String, Handle>;

pub type SummonerIconMap = HashMap<usize, Handle>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("failed to load file")]
    LoadingFile(#[from] Arc<io::Error>),
    #[error("failed to load image")]
    LoadingImage(#[from] Arc<image::ImageError>),
    #[error("failed to load sprite: {0}")]
    LoadingSprite(&'static str),
    #[error("failed to load data: {0}")]
    LoadingData(&'static str),
    #[error("failed to load json")]
    LoadingJSON(#[from] Arc<serde_json::Error>),
    #[error("invalid file name: {0:?}")]
    InvalidFileName(ffi::OsString),
}

#[derive(Debug, Clone)]
pub struct Assets {
    pub sprites: SpriteMap,
    pub data: DataMap,
    pub runes: RuneMap,
    pub emblems: EmblemMap,
    pub summoner_icons: SummonerIconMap,
}

impl Assets {
    const SUMMONER_ICONS_PATH: &'static str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/img/profileicon");
    const SPRITE_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/img/sprite");
    const DATA_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/data");
    const RUNES_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/img/runes");
    const EMBLEMS_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/img/emblems");

    pub fn load() -> Task<Message> {
        Task::batch(vec![
            Task::perform(Assets::new(), Message::AssetsLoaded),
            font::load(theme::ROBOTO_FLEX_TTF).map(Message::FontLoaded),
            font::load(theme::NOTO_SANS_TTF).map(Message::FontLoaded),
        ])
    }

    pub async fn new() -> Result<Assets, Error> {
        let summoner_icons = SummonerIconMap::default();

        let timer = Instant::now();
        let mut sprites = HashMap::default();
        let img_path = fs::read_dir(Assets::SPRITE_PATH).map_err(Arc::new)?;
        for sprite in img_path {
            let file = sprite.map_err(Arc::new)?;
            let sprite = {
                let name = file
                    .file_name()
                    .into_string()
                    .map_err(Error::InvalidFileName)?;
                if name.starts_with(".") {
                    continue;
                }

                tracing::info!("{name:?}");
                Sprite::try_from(name).map_err(Error::LoadingSprite)?
            };
            let image = image::ImageReader::open(file.path())
                .map_err(Arc::new)?
                .decode()
                .map_err(Arc::new)?;

            sprites.insert(sprite, image);
        }
        tracing::debug!("Loaded sprites in {:?}", timer.elapsed());

        let json_timer = Instant::now();
        let mut data = HashMap::default();
        let data_path = fs::read_dir(Assets::DATA_PATH).map_err(Arc::new)?;
        for data_dir in data_path {
            let file = data_dir.map_err(Arc::new)?;
            let sprite = {
                let name = file
                    .file_name()
                    .into_string()
                    .map_err(Error::InvalidFileName)?;
                DataFile::try_from(name).map_err(Error::LoadingData)?
            };
            let mut bytes = Vec::new();
            File::open(file.path())
                .map_err(Arc::new)?
                .read_to_end(&mut bytes)
                .map_err(Arc::new)?;
            let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(Arc::new)?;

            data.insert(sprite, value);
        }
        tracing::debug!("Loaded JSON data in {:?}", json_timer.elapsed());

        let runes_timer = Instant::now();
        let mut runes = HashMap::default();
        let value: serde_json::Value = serde_json::from_reader(
            File::open(format!("{}/{}", Assets::DATA_PATH, "runesReforged.json"))
                .map_err(Arc::new)?,
        )
        .map_err(Arc::new)?;

        for value in value.as_array().ok_or(Error::LoadingData("not an array"))? {
            let path = value["icon"]
                .as_str()
                .ok_or(Error::LoadingData("not a string"))?
                .trim_start_matches("perk-images/");
            let id = value["id"]
                .as_u64()
                .ok_or(Error::LoadingData("not a u64"))?;
            runes.insert(rune::Rune(id as usize), path.to_string());

            for slots in value["slots"]
                .as_array()
                .ok_or(Error::LoadingData("not an array"))?
            {
                for rune in slots["runes"]
                    .as_array()
                    .ok_or(Error::LoadingData("not an array"))?
                {
                    let path = rune["icon"]
                        .as_str()
                        .ok_or(Error::LoadingData("not a string"))?
                        .trim_start_matches("perk-images/");
                    let lesser_id = rune["id"]
                        .as_u64()
                        .ok_or(Error::LoadingData("not an array"))?;
                    runes.insert(rune::Rune(lesser_id as usize), path.to_string());
                }
            }
        }
        tracing::debug!("Loaded rune data in {:?}", runes_timer.elapsed());

        let emblem_timer = Instant::now();
        let mut emblems = HashMap::default();
        let img_path = fs::read_dir(Assets::EMBLEMS_PATH).map_err(Arc::new)?;
        for sprite in img_path {
            let file = sprite.map_err(Arc::new)?;
            let sprite = file
                .file_name()
                .into_string()
                .map_err(Error::InvalidFileName)?;
            let image = iced::widget::image::Handle::from_path(file.path());

            emblems.insert(sprite, image);
        }
        tracing::debug!("Loaded emblems in {:?}", emblem_timer.elapsed());
        tracing::debug!("Total time: {:?}", timer.elapsed());

        Ok(Assets {
            sprites,
            data,
            runes,
            emblems,
            summoner_icons,
        })
    }

    pub fn get_summoner_icon(&mut self, icon: usize) -> Handle {
        let path = format!("{}/{}.png", Assets::SUMMONER_ICONS_PATH, icon);

        tracing::debug!("Fetching icon at `{path}`");

        self.summoner_icons
            .entry(icon)
            .or_insert_with(|| Handle::from_path(path))
            .clone()
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
    Handle::from_rgba(icon.width(), icon.height(), icon.to_image().into_vec())
}

pub fn load_summoner_spell_icon(
    assets: &Assets,
    summoner_spell: game::player::SummonerSpell,
) -> Handle {
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
    Handle::from_rgba(icon.width(), icon.height(), icon.to_image().into_vec())
}

pub fn load_runes_icon(assets: &Assets, rune: rune::Rune) -> Handle {
    let rune_path = assets.runes.get(&rune).unwrap();
    let mut path = std::path::PathBuf::from(Assets::RUNES_PATH);
    path.push(rune_path);

    Handle::from_path(path)
}

pub fn load_item_icon(assets: &Assets, item: game::Item) -> Handle {
    let icon_data = assets.data.get(&DataFile::Item).unwrap();
    let icon = &icon_data["data"][item.0.to_string()]["image"];
    let sprite = Sprite::try_from(icon["sprite"].as_str().unwrap().to_string()).unwrap();
    let x = icon["x"].as_u64().unwrap() as u32;
    let y = icon["y"].as_u64().unwrap() as u32;
    let w = icon["w"].as_u64().unwrap() as u32;
    let h = icon["h"].as_u64().unwrap() as u32;
    let offset = 0;

    let icon_sprite = assets.sprites.get(&sprite).unwrap();
    let icon = icon_sprite.view(x + offset, y + offset, w - offset * 2, h - offset * 2);
    Handle::from_rgba(icon.width(), icon.height(), icon.to_image().into_vec())
}
