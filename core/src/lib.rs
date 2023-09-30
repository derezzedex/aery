pub mod client;
pub use client::Client;

pub mod summoner;
pub use summoner::Summoner;

pub mod game_match;
pub use game_match::GameMatch;

#[derive(Debug)]
pub enum Queue {
    /// CUSTOM
    Custom,
    /// SUMMONERS_RIFT_5V5_BLIND_PICK
    Blind,
    /// SUMMONERS_RIFT_5V5_DRAFT_PICK
    Draft,
    /// SUMMONERS_RIFT_5V5_RANKED_SOLO
    RankedSolo,
    /// SUMMONERS_RIFT_5V5_RANKED_FLEX
    RankedFlex,
    /// SUMMONERS_RIFT_CLASH
    Clash,
    /// HOWLING_ABYSS_5V5_ARAM,
    ARAM,

    BotIntro,
    BotBeginner,
    BotIntermediate,

    Other(u16),
    Unknown(u16),
}

impl From<Queue> for riven::consts::Queue {
    fn from(queue: Queue) -> riven::consts::Queue {
        match queue {
            Queue::Custom => riven::consts::Queue::CUSTOM,
            Queue::Blind => riven::consts::Queue::SUMMONERS_RIFT_5V5_BLIND_PICK,
            Queue::Draft => riven::consts::Queue::SUMMONERS_RIFT_5V5_DRAFT_PICK,
            Queue::RankedSolo => riven::consts::Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO,
            Queue::RankedFlex => riven::consts::Queue::SUMMONERS_RIFT_5V5_RANKED_FLEX,
            Queue::Clash => riven::consts::Queue::SUMMONERS_RIFT_CLASH,
            Queue::ARAM => riven::consts::Queue::HOWLING_ABYSS_5V5_ARAM,
            Queue::BotIntro => riven::consts::Queue::SUMMONERS_RIFT_CO_OP_VS_AI_INTRO_BOT,
            Queue::BotBeginner => riven::consts::Queue::SUMMONERS_RIFT_CO_OP_VS_AI_BEGINNER_BOT,
            Queue::BotIntermediate => {
                riven::consts::Queue::SUMMONERS_RIFT_CO_OP_VS_AI_INTERMEDIATE_BOT
            }
            Queue::Other(id) => riven::consts::Queue::from(id),
            Queue::Unknown(id) => riven::consts::Queue::from(id),
        }
    }
}

impl From<riven::consts::Queue> for Queue {
    fn from(queue: riven::consts::Queue) -> Queue {
        use riven::consts::Queue as ApiQueue;

        if !queue.is_known() {
            tracing::debug!("Unknown queue id encountered ({}).", queue.0);
            return Queue::Unknown(queue.0);
        }

        match queue {
            ApiQueue::CUSTOM => Queue::Custom,
            ApiQueue::SUMMONERS_RIFT_5V5_BLIND_PICK => Queue::Blind,
            ApiQueue::SUMMONERS_RIFT_5V5_DRAFT_PICK => Queue::Draft,
            ApiQueue::SUMMONERS_RIFT_5V5_RANKED_SOLO => Queue::RankedSolo,
            ApiQueue::SUMMONERS_RIFT_5V5_RANKED_FLEX => Queue::RankedFlex,
            ApiQueue::SUMMONERS_RIFT_CLASH => Queue::Clash,
            ApiQueue::HOWLING_ABYSS_5V5_ARAM => Queue::ARAM,
            ApiQueue::SUMMONERS_RIFT_CO_OP_VS_AI_INTRO_BOT => Queue::BotIntro,
            ApiQueue::SUMMONERS_RIFT_CO_OP_VS_AI_BEGINNER_BOT => Queue::BotBeginner,
            ApiQueue::SUMMONERS_RIFT_CO_OP_VS_AI_INTERMEDIATE_BOT => Queue::BotIntermediate,
            ApiQueue(id) => Queue::Other(id),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Duration(time::Duration);

#[derive(Debug, Clone)]
pub enum Role {
    Bottom,
    Jungle,
    Mid,
    Support,
    Top,
    Unknown,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Bottom => "Bottom",
            Role::Jungle => "Jungle",
            Role::Mid => "Mid",
            Role::Support => "Support",
            Role::Top => "Top",
            Role::Unknown => "Unknown",
        }
        .to_string()
    }
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_str() {
            "BOTTOM" => Role::Bottom,
            "JUNGLE" => Role::Jungle,
            "MIDDLE" => Role::Mid,
            "UTILITY" => Role::Support,
            "TOP" => Role::Top,
            _ => Role::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Item(u32);

impl TryFrom<i32> for Item {
    type Error = ();

    fn try_from(value: i32) -> Result<Item, Self::Error> {
        if value <= 0 {
            Err(())
        } else {
            Ok(Item(value as u32))
        }
    }
}

#[derive(Debug)]
pub struct Trinket(u32);

#[derive(Debug)]
pub struct Inventory([Option<Item>; 6]);

#[derive(Debug)]
pub struct Champion(u32);

impl From<riven::consts::Champion> for Champion {
    fn from(value: riven::consts::Champion) -> Self {
        Champion(value.0 as u32)
    }
}

#[derive(Debug)]
pub struct SummonerSpell(u32);

#[derive(Debug)]
pub struct SummonerSpells([SummonerSpell; 2]);

#[derive(Debug)]
pub struct PrimaryRune {
    keystone: u32,
    lesser: [u32; 3],
}

#[derive(Debug)]
pub struct SecondaryRune {
    lesser: [u32; 2],
}

#[derive(Debug)]
pub struct RuneStats {
    defense: u32,
    flex: u32,
    offense: u32,
}

impl From<riven::models::match_v5::PerkStats> for RuneStats {
    fn from(stats: riven::models::match_v5::PerkStats) -> Self {
        Self {
            defense: stats.defense as u32,
            flex: stats.flex as u32,
            offense: stats.offense as u32,
        }
    }
}

#[derive(Debug)]
pub struct RunePage {
    primary: PrimaryRune,
    secondary: SecondaryRune,
    stats: RuneStats,
}

#[derive(Debug)]
pub struct Participant {
    puuid: String,

    won: bool,
    role: Role,
    inventory: Inventory,
    trinket: Trinket,
    champion: Champion,
    summoner_spells: SummonerSpells,
    rune_page: RunePage,
    stats: ParticipantStats,
}

impl From<&riven::models::match_v5::Participant> for Participant {
    fn from(participant: &riven::models::match_v5::Participant) -> Self {
        let inventory = Inventory([
            Item::try_from(participant.item0).ok(),
            Item::try_from(participant.item1).ok(),
            Item::try_from(participant.item2).ok(),
            Item::try_from(participant.item3).ok(),
            Item::try_from(participant.item4).ok(),
            Item::try_from(participant.item5).ok(),
        ]);

        let rune_page = RunePage {
            primary: PrimaryRune {
                keystone: participant.perks.styles[0].selections[0].perk as u32,
                lesser: participant.perks.styles[0].selections[1..=3]
                    .iter()
                    .map(|s| s.perk as u32)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("failed to convert runes"),
            },
            secondary: SecondaryRune {
                lesser: participant.perks.styles[1].selections[0..=1]
                    .iter()
                    .map(|s| s.perk as u32)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("failed to convert runes"),
            },
            stats: participant.perks.stat_perks.clone().into(),
        };

        let stats = ParticipantStats {
            kills: participant.kills as u32,
            deaths: participant.deaths as u32,
            assists: participant.assists as u32,
            creep_score: participant.total_minions_killed as u32,
            vision_score: participant.vision_score as u32,
        };

        Self {
            puuid: participant.puuid.clone(),

            won: participant.win,
            role: participant.team_position.clone().into(),
            inventory,
            trinket: Trinket(participant.item6 as u32),
            champion: participant.champion().map_or(Champion(0), Champion::from),
            summoner_spells: SummonerSpells([
                SummonerSpell(participant.summoner1_id as u32),
                SummonerSpell(participant.summoner2_id as u32),
            ]),
            rune_page,
            stats,
        }
    }
}

#[derive(Debug)]
pub struct ParticipantStats {
    kills: u32,
    deaths: u32,
    assists: u32,
    creep_score: u32,
    vision_score: u32,
}

impl ParticipantStats {
    pub fn kda(&self) -> f32 {
        (self.kills + self.assists) as f32 / self.deaths as f32
    }

    pub fn kills(&self) -> u32 {
        self.kills
    }

    pub fn deaths(&self) -> u32 {
        self.deaths
    }

    pub fn assists(&self) -> u32 {
        self.assists
    }

    pub fn creep_score(&self) -> u32 {
        self.creep_score
    }

    pub fn vision_score(&self) -> u32 {
        self.vision_score
    }
}

impl Participant {
    pub fn puuid(&self) -> &str {
        &self.puuid
    }

    pub fn won(&self) -> bool {
        self.won
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn champion(&self) -> &Champion {
        &self.champion
    }

    pub fn summoner_spells(&self) -> &SummonerSpells {
        &self.summoner_spells
    }

    pub fn rune_page(&self) -> &RunePage {
        &self.rune_page
    }

    pub fn stats(&self) -> &ParticipantStats {
        &self.stats
    }
}
