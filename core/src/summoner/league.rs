use crate::game;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct League(pub(crate) riven::models::league_v4::LeagueEntry);

impl League {
    pub fn queue_kind(&self) -> game::Queue {
        use riven::consts::QueueType;

        match self.0.queue_type {
            QueueType::RANKED_SOLO_5x5 => game::Queue::RankedSolo,
            QueueType::RANKED_FLEX_SR => game::Queue::RankedFlex,
            _ => game::Queue::Unknown(0),
        }
    }

    pub fn tier(&self) -> Option<Tier> {
        let points = self.points() as u16;
        let division = self.division();

        self.0
            .tier
            .filter(|&t| t != riven::consts::Tier::UNRANKED)
            .map(|tier| match tier {
                riven::consts::Tier::UNRANKED => unreachable!(),
                riven::consts::Tier::IRON => Tier::Iron(division.unwrap()),
                riven::consts::Tier::BRONZE => Tier::Bronze(division.unwrap()),
                riven::consts::Tier::SILVER => Tier::Silver(division.unwrap()),
                riven::consts::Tier::GOLD => Tier::Gold(division.unwrap()),
                riven::consts::Tier::PLATINUM => Tier::Platinum(division.unwrap()),
                riven::consts::Tier::EMERALD => Tier::Emerald(division.unwrap()),
                riven::consts::Tier::DIAMOND => Tier::Diamond(division.unwrap()),
                riven::consts::Tier::MASTER => Tier::Master(points),
                riven::consts::Tier::GRANDMASTER => Tier::Grandmaster(points),
                riven::consts::Tier::CHALLENGER => Tier::Challenger(points),
            })
    }

    #[allow(deprecated)]
    pub fn division(&self) -> Option<Division> {
        self.0
            .rank
            .filter(|&d| d != riven::consts::Division::V)
            .map(|division| match division {
                riven::consts::Division::I => Division::One(self.0.league_points as u8),
                riven::consts::Division::II => Division::Two(self.0.league_points as u8),
                riven::consts::Division::III => Division::Three(self.0.league_points as u8),
                riven::consts::Division::IV => Division::Four(self.0.league_points as u8),
                _ => unreachable!(),
            })
    }

    pub fn points(&self) -> u32 {
        self.0.league_points as u32
    }
    pub fn wins(&self) -> u32 {
        self.0.wins as u32
    }
    pub fn losses(&self) -> u32 {
        self.0.losses as u32
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Tier {
    Iron(Division),
    Bronze(Division),
    Silver(Division),
    Gold(Division),
    Platinum(Division),
    Emerald(Division),
    Diamond(Division),
    Master(u16),
    Grandmaster(u16),
    Challenger(u16),
}

impl Tier {
    pub fn points(&self) -> u16 {
        match self {
            Tier::Challenger(points) | Tier::Grandmaster(points) | Tier::Master(points) => *points,
            Tier::Iron(division)
            | Tier::Bronze(division)
            | Tier::Silver(division)
            | Tier::Gold(division)
            | Tier::Platinum(division)
            | Tier::Emerald(division)
            | Tier::Diamond(division) => match division {
                Division::One(points)
                | Division::Two(points)
                | Division::Three(points)
                | Division::Four(points) => *points as u16,
            },
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Division {
    One(u8),
    Two(u8),
    Three(u8),
    Four(u8),
}
