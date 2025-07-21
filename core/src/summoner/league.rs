use riven::models::league_v4;

#[derive(Debug, Clone, PartialEq, Eq, bitcode::Encode, bitcode::Decode)]
pub enum Kind {
    SummonersRift(SummonersRift),
    TeamfightTactics(TeamfightTactics),
    Arena,
    Unknown(String),
}

impl From<riven::consts::QueueType> for Kind {
    fn from(queue: riven::consts::QueueType) -> Self {
        use riven::consts::QueueType;

        #[allow(deprecated)]
        match queue {
            QueueType::CHERRY => Kind::Arena,
            QueueType::RANKED_SOLO_5x5 => Kind::SummonersRift(SummonersRift::Solo),
            QueueType::RANKED_FLEX_SR => Kind::SummonersRift(SummonersRift::Flex),
            QueueType::RANKED_FLEX_TT => Kind::SummonersRift(SummonersRift::TwistedTreeline),
            QueueType::RANKED_TFT => Kind::TeamfightTactics(TeamfightTactics::Ranked),
            QueueType::RANKED_TFT_DOUBLE_UP | QueueType::RANKED_TFT_PAIRS => {
                Kind::TeamfightTactics(TeamfightTactics::DoubleUp)
            }
            QueueType::RANKED_TFT_TURBO => Kind::TeamfightTactics(TeamfightTactics::Turbo),
            QueueType::UNKNOWN(name) => Kind::Unknown(name),
            unknown => Kind::Unknown(String::from(unknown.as_ref())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, bitcode::Encode, bitcode::Decode)]
pub enum SummonersRift {
    Solo,
    Flex,
    TwistedTreeline,
}

#[derive(Debug, Clone, PartialEq, Eq, bitcode::Encode, bitcode::Decode)]
pub enum TeamfightTactics {
    Ranked,
    Turbo,
    DoubleUp,
}

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct League {
    pub kind: Kind,
    pub tier: Option<Tier>,
    pub wins: u32,
    pub losses: u32,
}

impl League {
    pub fn division(&self) -> Option<Division> {
        self.tier.as_ref().map(Tier::division).flatten()
    }

    pub fn points(&self) -> u16 {
        self.tier.as_ref().map(Tier::points).unwrap_or(0)
    }
}

impl From<league_v4::LeagueEntry> for League {
    fn from(league: league_v4::LeagueEntry) -> Self {
        let wins = league.wins as u32;
        let losses = league.losses as u32;
        let kind = Kind::from(league.queue_type.clone());
        let tier = Tier::try_from(league).ok();

        Self {
            kind,
            tier,
            wins,
            losses,
        }
    }
}

#[derive(Debug, Copy, Clone, bitcode::Encode, bitcode::Decode)]
pub enum Division {
    One(u8),
    Two(u8),
    Three(u8),
    Four(u8),
}

#[derive(Debug, Copy, Clone, bitcode::Encode, bitcode::Decode)]
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
    pub fn division(&self) -> Option<Division> {
        match self {
            Tier::Challenger(_) | Tier::Grandmaster(_) | Tier::Master(_) => None,
            Tier::Iron(division)
            | Tier::Bronze(division)
            | Tier::Silver(division)
            | Tier::Gold(division)
            | Tier::Platinum(division)
            | Tier::Emerald(division)
            | Tier::Diamond(division) => Some(*division),
        }
    }

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

impl TryFrom<league_v4::LeagueEntry> for Tier {
    type Error = ();

    fn try_from(league: league_v4::LeagueEntry) -> Result<Self, ()> {
        let points = league.league_points as u16;

        #[allow(deprecated)]
        let division = league
            .rank
            .filter(|&d| d != riven::consts::Division::V)
            .map(|division| match division {
                riven::consts::Division::I => Division::One(points as u8),
                riven::consts::Division::II => Division::Two(points as u8),
                riven::consts::Division::III => Division::Three(points as u8),
                riven::consts::Division::IV => Division::Four(points as u8),
                _ => unreachable!(),
            });

        league
            .tier
            .map(|tier| match tier {
                riven::consts::Tier::UNRANKED => None,
                riven::consts::Tier::IRON => Some(Tier::Iron(division.unwrap())),
                riven::consts::Tier::BRONZE => Some(Tier::Bronze(division.unwrap())),
                riven::consts::Tier::SILVER => Some(Tier::Silver(division.unwrap())),
                riven::consts::Tier::GOLD => Some(Tier::Gold(division.unwrap())),
                riven::consts::Tier::PLATINUM => Some(Tier::Platinum(division.unwrap())),
                riven::consts::Tier::EMERALD => Some(Tier::Emerald(division.unwrap())),
                riven::consts::Tier::DIAMOND => Some(Tier::Diamond(division.unwrap())),
                riven::consts::Tier::MASTER => Some(Tier::Master(points)),
                riven::consts::Tier::GRANDMASTER => Some(Tier::Grandmaster(points)),
                riven::consts::Tier::CHALLENGER => Some(Tier::Challenger(points)),
            })
            .flatten()
            .ok_or(())
    }
}
