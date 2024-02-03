#![allow(dead_code)]

pub mod game;
pub mod ranked_overview;
pub mod search_bar;
pub mod summoner;
pub mod timeline;

use crate::core;
use crate::theme;
use iced::widget::image::Handle;

#[derive(Debug, Clone)]
enum Queue {
    RankedFlex,
    RankedSolo,
}

impl ToString for Queue {
    fn to_string(&self) -> String {
        match self {
            Queue::RankedFlex => "Ranked Flex",
            Queue::RankedSolo => "Ranked Solo",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Role {
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
struct Item(u16);

#[derive(Debug, Clone)]
struct Inventory([Option<Item>; 6]);

#[derive(Debug, Clone)]
struct Summoner(String);

impl ToString for Summoner {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

mod formatting {
    pub fn win(win: bool) -> String {
        if win { "Victory" } else { "Defeat" }.to_string()
    }

    pub fn kda(kills: u16, deaths: u16, assists: u16) -> String {
        let kda = (kills as f32 + assists as f32) / deaths as f32;
        format!("{kda:.2} KDA")
    }

    pub fn creep_score(creep_score: u16, minutes: u16) -> String {
        let cs_per_minute = creep_score as f32 / minutes as f32;

        format!("{creep_score} CS ({cs_per_minute:.1})")
    }

    pub fn vision_score(vision_score: u16) -> String {
        format!("{vision_score} vision")
    }
}
