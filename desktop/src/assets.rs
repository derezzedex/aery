use aery_core::summoner;
use iced::widget::image::Handle;
use iced::Task;

use std::collections::HashMap;

use crate::core::assets::emblem;
use crate::{core, Message};

type HandleMap<T> = HashMap<T, Handle>;

#[derive(Debug, Clone)]
pub struct Assets {
    champion: HandleMap<core::Champion>,
    rune: HandleMap<core::Rune>,
    spell: HandleMap<core::SummonerSpell>,
    item: HandleMap<core::Item>,
    emblem: HandleMap<emblem::Id>,
}

impl Assets {
    pub fn load() -> Task<Message> {
        Task::perform(Assets::new(), Message::AssetsLoaded)
    }

    pub async fn new() -> Result<Assets, String> {
        let bytes = fetch().await?;
        let unloaded = core::Assets::decode(bytes);

        let champion = unloaded
            .champion
            .0
            .into_iter()
            .map(|(id, asset)| {
                (
                    core::Champion::new(id.0 as u32),
                    Handle::from_bytes(asset.icon.0),
                )
            })
            .collect();
        let rune = unloaded
            .rune
            .0
            .into_iter()
            .map(|(id, asset)| (core::Rune(id.0 as usize), Handle::from_bytes(asset.icon.0)))
            .collect();
        let spell = unloaded
            .spell
            .0
            .into_iter()
            .map(|(id, asset)| {
                (
                    core::SummonerSpell::new(id.0),
                    Handle::from_bytes(asset.icon.0),
                )
            })
            .collect();
        let item = unloaded
            .item
            .0
            .into_iter()
            .map(|(id, asset)| (core::Item(id.0 as usize), Handle::from_bytes(asset.icon.0)))
            .collect();
        let emblem = unloaded
            .emblem
            .0
            .into_iter()
            .map(|(id, asset)| (id, Handle::from_bytes(asset.icon.0)))
            .collect();

        Ok(Assets {
            champion,
            rune,
            spell,
            item,
            emblem,
        })
    }

    pub fn champion(&self, id: &core::Champion) -> Handle {
        self.champion.get(id).cloned().unwrap()
    }

    pub fn rune(&self, id: &core::Rune) -> Handle {
        self.rune.get(id).cloned().unwrap()
    }

    pub fn spell(&self, id: &core::SummonerSpell) -> Handle {
        self.spell.get(id).cloned().unwrap()
    }

    pub fn item(&self, id: &core::Item) -> Handle {
        self.item.get(id).cloned().unwrap()
    }

    pub fn emblem(&self, tier: &summoner::Tier) -> Handle {
        let id = emblem::Id::from_tier(tier);
        self.emblem.get(&id).cloned().unwrap()
    }
}

pub async fn fetch() -> Result<Vec<u8>, String> {
    use futures::TryFutureExt;

    let worker_url = dotenv_codegen::dotenv!("WORKER_URL");
    let path = format!("{worker_url}/assets/latest");
    tracing::info!("Requesting assets to {path}");

    reqwest::get(path)
        .map_err(|e| e.to_string())
        .await?
        .bytes()
        .map_err(|e| e.to_string())
        .await
        .map(|b| b.to_vec())
}
