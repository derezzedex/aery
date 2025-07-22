use crate::account;
use crate::game;
use crate::game::item;
use crate::game::rune;
use crate::{Champion, Team};

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct Player {
    pub puuid: String,
    pub riot_id: account::RiotId,

    pub team: Team,
    pub result: game::Result,
    pub role: Option<game::Role>,
    pub inventory: item::Inventory,
    pub trinket: item::Trinket,
    pub champion: Champion,
    pub summoner_spells: SummonerSpells,
    pub rune_page: rune::Page,
    pub stats: Stats,
}

impl Player {
    #[cfg(feature = "dummy")]
    pub fn dummy(
        riot_id: account::RiotId,
        team: Team,
        role: game::Role,
        champion: Champion,
        result: game::Result,
    ) -> Self {
        use account::Account;
        use item::Item;
        use rune::Rune;

        let account = Account::dummy(riot_id);

        Player {
            puuid: account.puuid.as_ref().to_string(),
            riot_id: account.riot_id,
            team,
            result,
            role: Some(role),
            inventory: item::Inventory {
                items: [
                    Some(Item(3161)),
                    Some(Item(6692)),
                    Some(Item(3156)),
                    Some(Item(6333)),
                    Some(Item(3174)),
                    Some(Item(6695)),
                ],
            },
            trinket: item::Trinket(3364),
            champion,
            summoner_spells: SummonerSpells([SummonerSpell(11), SummonerSpell(4)]),
            rune_page: rune::Page {
                primary: rune::path::Primary {
                    path: rune::path::Kind::Precision,
                    keystone: rune::path::Keystone { rune: Rune(8010) },
                    runes: [Rune(9111), Rune(9105), Rune(8017)],
                },
                secondary: rune::path::Secondary {
                    path: rune::path::Kind::Inspiration,
                    runes: [Rune(8304), Rune(8347)],
                },
                shards: rune::Shards {
                    offense: rune::Shard::AbilityHaste,
                    flex: rune::Shard::AdaptiveForce,
                    defense: rune::Shard::HealthScaling,
                },
            },
            stats: Stats {
                level: 18,
                kills: 14,
                deaths: 4,
                assists: 6,
                creep_score: 33,
                monster_score: 199,
                vision_score: 36,
                damage_dealt: 27476,
                damage_taken: 39444,
                gold: 17258,
                control_wards: 4,
                wards_placed: 11,
                wards_removed: 2,
            },
        }
    }
}

impl From<&riven::models::match_v5::Participant> for Player {
    fn from(participant: &riven::models::match_v5::Participant) -> Self {
        let inventory = item::Inventory::from(
            [
                participant.item0,
                participant.item1,
                participant.item2,
                participant.item3,
                participant.item4,
                participant.item5,
            ]
            .map(game::Item::try_from)
            .map(Result::ok),
        );

        let stats = Stats {
            level: participant.champ_level as u32,

            kills: participant.kills as u32,
            deaths: participant.deaths as u32,
            assists: participant.assists as u32,
            creep_score: participant.total_minions_killed as u32,
            monster_score: participant.neutral_minions_killed as u32,
            vision_score: participant.vision_score as u32,

            damage_dealt: participant.total_damage_dealt_to_champions as u32,
            damage_taken: participant.total_damage_taken as u32,
            gold: participant.gold_earned as u32,

            control_wards: participant.vision_wards_bought_in_game as u32,
            wards_placed: participant.wards_placed as u32,
            wards_removed: participant.wards_killed as u32,
        };

        let result = if participant.game_ended_in_early_surrender {
            game::Result::Remake
        } else if participant.game_ended_in_surrender {
            game::Result::Surrender
        } else if participant.win {
            game::Result::Victory
        } else {
            game::Result::Defeat
        };

        Self {
            puuid: participant.puuid.clone(),
            riot_id: account::RiotId {
                name: participant.riot_id_game_name.clone(),
                tagline: participant.riot_id_tagline.clone(),
            },

            team: Team(participant.team_id as usize),
            result,
            role: game::Role::try_from(&participant.team_position).ok(),
            inventory,
            trinket: item::Trinket(participant.item6 as usize),
            champion: participant.champion().map_or(Champion(0), Champion::from),
            summoner_spells: SummonerSpells([
                SummonerSpell(participant.summoner1_id as u32),
                SummonerSpell(participant.summoner2_id as u32),
            ]),
            rune_page: rune::Page::from(participant.perks.clone()),
            stats,
        }
    }
}

#[derive(Debug, Clone, Copy, bitcode::Encode, bitcode::Decode)]
pub struct Stats {
    pub level: u32,

    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,

    pub creep_score: u32,
    pub monster_score: u32,
    pub vision_score: u32,

    pub damage_dealt: u32,
    pub damage_taken: u32,
    pub gold: u32,

    pub control_wards: u32,
    pub wards_placed: u32,
    pub wards_removed: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bitcode::Encode, bitcode::Decode)]
pub struct SummonerSpell(u32);

impl SummonerSpell {
    pub fn new(id: u32) -> Self {
        //TODO: verify id
        Self(id)
    }
    pub fn id(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, bitcode::Encode, bitcode::Decode)]
pub struct SummonerSpells(pub [SummonerSpell; 2]);

impl SummonerSpells {
    pub fn first(&self) -> SummonerSpell {
        self.0[0]
    }

    pub fn second(&self) -> SummonerSpell {
        self.0[1]
    }
}

impl From<[SummonerSpell; 2]> for SummonerSpells {
    fn from(spells: [SummonerSpell; 2]) -> Self {
        Self(spells)
    }
}
