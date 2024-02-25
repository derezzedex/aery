use crate::game;
use crate::game::rune;
use crate::summoner;
use crate::{Champion, Inventory, Item, Role, SummonerSpell, SummonerSpells, Team, Trinket};

#[derive(Debug, Clone)]
pub struct Player {
    pub puuid: String,
    pub name: String,
    pub riot_id: summoner::RiotId,

    pub team: Team,
    pub result: game::Result,
    pub role: Role,
    pub inventory: Inventory,
    pub trinket: Trinket,
    pub champion: Champion,
    pub summoner_spells: SummonerSpells,
    pub rune_page: rune::Page,
    pub stats: Stats,
}

impl From<&riven::models::match_v5::Participant> for Player {
    fn from(participant: &riven::models::match_v5::Participant) -> Self {
        let inventory = Inventory([
            Item::try_from(participant.item0).ok(),
            Item::try_from(participant.item1).ok(),
            Item::try_from(participant.item2).ok(),
            Item::try_from(participant.item3).ok(),
            Item::try_from(participant.item4).ok(),
            Item::try_from(participant.item5).ok(),
        ]);

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
            name: participant.summoner_name.clone(),
            riot_id: summoner::RiotId {
                name: participant.riot_id_game_name.clone(),
                tagline: participant.riot_id_tagline.clone(),
            },

            team: Team(participant.team_id as usize),
            result,
            role: participant.team_position.clone().into(),
            inventory,
            trinket: Trinket(participant.item6 as u32),
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

#[derive(Debug, Clone, Copy)]
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
