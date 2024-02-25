pub mod path {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Kind {
        Precision,
        Domination,
        Sorcery,
        Inspiration,
        Resolve,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Rune(pub usize);

    impl From<Kind> for Rune {
        fn from(kind: Kind) -> Self {
            match kind {
                Kind::Precision => Rune(8000),
                Kind::Domination => Rune(8100),
                Kind::Sorcery => Rune(8200),
                Kind::Inspiration => Rune(8300),
                Kind::Resolve => Rune(8400),
            }
        }
    }

    impl From<Rune> for Kind {
        fn from(rune: Rune) -> Self {
            // TODO: verify this, caused by https://github.com/RiotGames/developer-relations/issues/724
            // this should transform the "lesser" rune id into the "major", by zeroing out the last two digits

            // NOTE: Surprisingly this broke rather easily, who would've guessed?
            // Obviously Riot Games is cooking something with their whole item `id` "allocation";
            // This `match` will need to be updated everytime the API changes; but then ideally
            // I would check the API and make changes myself. Soon the whole `core` API should
            // follow this, by using constants and enums instead of `u32`.
            match rune.0 {
                8000..=8099 => Kind::Precision,
                8100..=8199 => Kind::Domination,
                8200..=8299 => Kind::Sorcery,
                8300..=8399 => Kind::Inspiration,
                8400..=8499 => Kind::Resolve,

                9923 => Kind::Domination, // HailfOfBlades,
                9101 | 9103 | 9104 | 9105 | 9111 => Kind::Precision, // Overheal | Triumph | LegendAlacrity | LegendTenacity | LegendBloodline,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Keystone {
        pub rune: Rune,
    }

    impl From<Rune> for Keystone {
        fn from(rune: Rune) -> Self {
            Self { rune }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Primary {
        pub path: Kind,
        pub keystone: Keystone,
        pub runes: [Rune; 3],
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Secondary {
        pub path: Kind,
        pub runes: [Rune; 2],
    }
}

pub use path::Rune;

#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub primary: path::Primary,
    pub secondary: path::Secondary,
    pub shards: Shards,
}

#[derive(Debug, Clone, Copy)]
pub struct Shards {
    pub offense: Shard,
    pub flex: Shard,
    pub defense: Shard,
}

#[derive(Debug, Clone, Copy)]
pub enum Shard {
    HealthScaling = 5001,
    Armor = 5002,
    MagicResist = 5003,
    AttackSpeed = 5005,
    AbilityHaste = 5007,
    AdaptiveForce = 5008,
    MoveSpeed = 5010,
    Health = 5011,
    ResistScaling = 5012,
    Tenacity = 5013,
}

impl From<riven::models::match_v5::Perks> for Page {
    fn from(perks: riven::models::match_v5::Perks) -> Self {
        let primary_rune = Rune(perks.styles[0].selections[0].perk as usize);
        let secondary_rune = Rune(perks.styles[1].selections[0].perk as usize);

        Self {
            primary: path::Primary {
                path: path::Kind::from(primary_rune),
                keystone: path::Keystone::from(primary_rune),
                runes: perks.styles[0]
                    .selections
                    .iter()
                    .skip(1)
                    .map(|s| Rune(s.perk as usize))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            },
            secondary: path::Secondary {
                path: path::Kind::from(secondary_rune),
                runes: perks.styles[1].selections[0..=1]
                    .iter()
                    .map(|s| Rune(s.perk as usize))
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("failed to convert runes"),
            },
            shards: perks.stat_perks.into(),
        }
    }
}

impl TryFrom<usize> for Shard {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == Shard::HealthScaling as usize => Ok(Shard::HealthScaling),
            x if x == Shard::Armor as usize => Ok(Shard::Armor),
            x if x == Shard::MagicResist as usize => Ok(Shard::MagicResist),
            x if x == Shard::AttackSpeed as usize => Ok(Shard::AttackSpeed),
            x if x == Shard::AbilityHaste as usize => Ok(Shard::AbilityHaste),
            x if x == Shard::AdaptiveForce as usize => Ok(Shard::AdaptiveForce),
            x if x == Shard::MoveSpeed as usize => Ok(Shard::MoveSpeed),
            x if x == Shard::Health as usize => Ok(Shard::Health),
            x if x == Shard::ResistScaling as usize => Ok(Shard::ResistScaling),
            x if x == Shard::Tenacity as usize => Ok(Shard::Tenacity),
            _ => Err(()),
        }
    }
}

impl From<riven::models::match_v5::PerkStats> for Shards {
    fn from(statmods: riven::models::match_v5::PerkStats) -> Self {
        Self {
            offense: Shard::try_from(statmods.offense as usize).unwrap(),
            flex: Shard::try_from(statmods.flex as usize).unwrap(),
            defense: Shard::try_from(statmods.defense as usize).unwrap(),
        }
    }
}
