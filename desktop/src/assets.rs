use iced::widget::image::Handle;
use image::GenericImageView;

use std::collections::HashMap;

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

pub type RuneMap = HashMap<String, String>;

pub type EmblemMap = HashMap<String, Handle>;

pub struct Assets {
    pub sprites: SpriteMap,
    pub data: DataMap,
    pub runes: RuneMap,
    pub emblems: EmblemMap,
}

// TODO: use champion id instead of name
pub fn load_champion_icon(assets: &Assets, champion: &str) -> Handle {
    let icon_data = assets.data.get(&DataFile::Champion).unwrap();
    let icon = &icon_data["data"][champion]["image"];
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

pub fn load_summoner_spell_icon(assets: &Assets, summoner_spell: &str) -> Handle {
    let icon_data = assets.data.get(&DataFile::SummonerSpell).unwrap();
    let icon = &icon_data["data"][summoner_spell]["image"];
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

pub fn load_runes_icon(assets: &Assets, rune: &str) -> Handle {
    let rune_path = assets.runes.get(rune).unwrap();
    let mut path = std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\assets\\img\\runes\\"
    ));
    path.push(rune_path);

    Handle::from_path(path)
}

pub fn load_item_icon(assets: &Assets, item_id: &str) -> Handle {
    let icon_data = assets.data.get(&DataFile::Item).unwrap();
    let icon = &icon_data["data"][item_id]["image"];
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
