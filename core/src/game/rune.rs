#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Keystone(pub u32);

impl Keystone {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Primary {
    pub keystone: Keystone,
}

impl Primary {
    pub fn keystone(&self) -> Keystone {
        self.keystone
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Secondary {
    pub lesser: [Keystone; 2],
}

impl Secondary {
    pub fn keystone(&self) -> Keystone {
        // TODO: verify this, caused by https://github.com/RiotGames/developer-relations/issues/724
        // this should transform the "lesser" rune id into the "major", by zeroing out the last two digits

        // NOTE: Surprisingly this broke rather easily, who would've guessed?
        // Obviously Riot Games is cooking something with their whole item `id` "allocation";
        // This `match` will need to be updated everytime the API changes; but then ideally
        // I would check the API and make changes myself. Soon the whole `core` API should
        // follow this, by using constants and enums instead of `u32`.
        let id = self.lesser[0].0;
        match id {
            9923 => Keystone(8100), // HailfOfBlades => Domination,
            9101 | 9111 | 9104 | 9105 | 9103 => Keystone(8000), // Overheal | Triumph | LegendAlacrity | LegendTenacity | LegendBloodline => Precision,
            _ => Keystone((id / 100) * 100),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub primary: Primary,
    pub secondary: Secondary,
}
