use crate::core;
use iced::widget::image::Handle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Bottom,
    Jungle,
    Mid,
    Support,
    Top,
}

impl TryFrom<core::Role> for Role {
    type Error = ();

    fn try_from(role: core::Role) -> Result<Self, Self::Error> {
        match role {
            core::Role::Bottom => Ok(Self::Bottom),
            core::Role::Jungle => Ok(Self::Jungle),
            core::Role::Mid => Ok(Self::Mid),
            core::Role::Support => Ok(Self::Support),
            core::Role::Top => Ok(Self::Top),
            core::Role::Unknown => Err(()),
        }
    }
}

impl Role {
    pub fn icon(&self) -> Handle {
        let role = self.to_string().to_ascii_lowercase();
        let path = format!(
            "{}\\assets\\img\\position\\{role}.png",
            env!("CARGO_MANIFEST_DIR"),
        );

        Handle::from_path(path)
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Bottom => "Bottom",
            Role::Jungle => "Jungle",
            Role::Mid => "Mid",
            Role::Support => "Support",
            Role::Top => "Top",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Champion(u16);

#[derive(Debug, Clone, Copy)]
pub struct Item(u16);

#[derive(Debug, Clone)]
pub struct Inventory([Option<Item>; 6]);

#[derive(Debug, Clone)]
pub struct Summoner(pub String);

impl ToString for Summoner {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub mod formatting {
    use crate::core;

    pub fn win(result: core::GameResult) -> String {
        match result {
            core::GameResult::Remake => "Remake",
            core::GameResult::Defeat | core::GameResult::Surrender => "Defeat",
            core::GameResult::Victory => "Victory",
        }
        .to_string()
    }

    pub fn kda(kills: u32, deaths: u32, assists: u32) -> String {
        let mut kda = (kills as f32 + assists as f32) / deaths as f32;
        if !kda.is_normal() {
            kda = 0.0;
        }
        format!("{kda:.2} KDA")
    }

    pub fn creep_score(creep_score: u32, minutes: u32) -> String {
        let mut cs_per_minute = creep_score as f32 / minutes as f32;
        if !cs_per_minute.is_normal() {
            cs_per_minute = 0.0;
        }
        format!("{creep_score} CS ({cs_per_minute:.1})")
    }

    pub fn vision_score(vision_score: u32) -> String {
        format!("{vision_score} vision")
    }
}
