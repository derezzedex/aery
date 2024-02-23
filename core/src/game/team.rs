#[derive(Debug, Clone)]
pub struct Team {
    pub won: bool,
    pub id: Id,
    pub bans: Vec<Ban>,
    pub objectives: Objectives,
}

impl From<riven::models::match_v5::Team> for Team {
    fn from(team: riven::models::match_v5::Team) -> Self {
        let bans = team
            .bans
            .into_iter()
            .map(|ban| Ban {
                champion: ban.champion_id.into(),
                turn_picked: ban.pick_turn as usize,
            })
            .collect();

        Team {
            won: team.win,
            id: Id::from(team.team_id),
            bans,
            objectives: Objectives::from(team.objectives),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(usize);

impl From<riven::consts::Team> for Id {
    fn from(team: riven::consts::Team) -> Self {
        Self(team as usize)
    }
}

impl Id {
    pub const BLUE: Id = Id(100);
    pub const RED: Id = Id(200);
}

#[derive(Debug, Clone)]
pub struct Ban {
    pub champion: crate::Champion,
    pub turn_picked: usize,
}

#[derive(Debug, Clone)]
pub struct Objectives {
    pub baron: Objective,
    pub champion: Objective,
    pub dragon: Objective,
    pub inhibitor: Objective,
    pub rift_herald: Objective,
    pub tower: Objective,
    pub horde: Option<Objective>,
}

impl Default for Objectives {
    fn default() -> Self {
        Self {
            baron: Objective::default(),
            champion: Objective::default(),
            dragon: Objective::default(),
            inhibitor: Objective::default(),
            rift_herald: Objective::default(),
            tower: Objective::default(),
            horde: Some(Objective::default()),
        }
    }
}

impl From<riven::models::match_v5::Objectives> for Objectives {
    fn from(objectives: riven::models::match_v5::Objectives) -> Self {
        Self {
            baron: Objective {
                first: objectives.baron.first,
                kills: objectives.baron.kills as usize,
            },
            champion: Objective {
                first: objectives.champion.first,
                kills: objectives.champion.kills as usize,
            },
            dragon: Objective {
                first: objectives.dragon.first,
                kills: objectives.dragon.kills as usize,
            },
            inhibitor: Objective {
                first: objectives.inhibitor.first,
                kills: objectives.inhibitor.kills as usize,
            },
            rift_herald: Objective {
                first: objectives.rift_herald.first,
                kills: objectives.rift_herald.kills as usize,
            },
            tower: Objective {
                first: objectives.tower.first,
                kills: objectives.tower.kills as usize,
            },
            horde: objectives.horde.map(|horde| Objective {
                first: horde.first,
                kills: horde.kills as usize,
            }),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Objective {
    pub first: bool,
    pub kills: usize,
}
