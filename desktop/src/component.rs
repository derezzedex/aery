#![allow(dead_code)]

pub mod game;
pub mod ranked_overview;
pub mod search_bar;
pub mod summoner;
pub mod timeline;

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

#[derive(Debug, Clone)]
enum Role {
    Bottom,
    Jungle,
    Mid,
    Support,
    Top,
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
struct Time(time::OffsetDateTime);

impl ToString for Time {
    fn to_string(&self) -> String {
        let now = time::OffsetDateTime::now_utc();
        let duration = now - self.0;
        let seconds = duration.whole_seconds();
        let minutes = seconds / 60;
        let hours = minutes / 60;
        let days = hours / 24;
        let weeks = days / 7;
        let months = days / 30;
        let years = days / 365;

        if seconds < 60 {
            String::from("few seconds ago")
        } else if minutes < 60 {
            format!(
                "{} minute{} ago",
                minutes,
                if minutes == 1 { "" } else { "s" }
            )
        } else if hours < 24 {
            format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
        } else if days < 7 {
            if days == 1 {
                String::from("yesterday")
            } else {
                format!("{} days ago", days)
            }
        } else if weeks < 4 {
            if weeks == 1 {
                String::from("last week")
            } else {
                format!("{} weeks ago", weeks)
            }
        } else if months < 12 {
            if months == 1 {
                String::from("last month")
            } else {
                format!("{} months ago", months)
            }
        } else {
            if years == 1 {
                return String::from("last year");
            } else {
                format!("{} years ago", years)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Duration(time::Duration);

impl ToString for Duration {
    fn to_string(&self) -> String {
        let minutes = self.0.whole_minutes();
        let seconds = self.0.whole_seconds();

        format!("{minutes}:{seconds}")
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
