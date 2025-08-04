use aery_core::summoner;
use iced::Task;
use iced::widget::image::Handle;

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::core::assets::emblem;
use crate::{Message, core};

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
                    Handle::from_bytes(asset.icon),
                )
            })
            .collect();
        let rune = unloaded
            .rune
            .0
            .into_iter()
            .map(|(id, asset)| (core::Rune(id.0 as usize), Handle::from_bytes(asset.icon)))
            .collect();
        let spell = unloaded
            .spell
            .0
            .into_iter()
            .map(|(id, asset)| {
                (
                    core::SummonerSpell::new(id.0),
                    Handle::from_bytes(asset.icon),
                )
            })
            .collect();
        let item = unloaded
            .item
            .0
            .into_iter()
            .map(|(id, asset)| (core::Item(id.0 as usize), Handle::from_bytes(asset.icon)))
            .collect();
        let emblem = unloaded
            .emblem
            .0
            .into_iter()
            .map(|(id, asset)| (id, Handle::from_bytes(asset.icon)))
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
        self.rune.get(id).cloned().unwrap_or_else(|| missing())
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

pub fn missing() -> Handle {
    static HANDLE: LazyLock<Handle> = LazyLock::new(|| {
        Handle::from_path(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/img/missing.png"
        ))
    });

    HANDLE.clone()
}

#[cfg(not(feature = "dummy"))]
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

#[cfg(feature = "dummy")]
pub async fn fetch() -> Result<Vec<u8>, String> {
    std::fs::read("assets/dummy/assets.aery").map_err(|error| error.to_string())
}
