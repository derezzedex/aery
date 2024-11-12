use crate::{Error, Result};
use worker::kv;

pub async fn get<T: serde::de::DeserializeOwned>(kv: &kv::KvStore, id: &str) -> Result<Option<T>> {
    kv.get(id)
        .text()
        .await
        .map_err(Error::from_string)
        .map(|text| {
            text.as_deref()
                .map(serde_json::from_str)
                .map(|res| res.map_err(Error::from_string))
        })
        .map(Option::transpose)?
}

pub async fn insert<T: serde::Serialize>(kv: &kv::KvStore, id: &str, data: &T) -> Result<()> {
    let json = serde_json::to_string(&data).map_err(Error::from_string)?;

    kv.put(id, json)
        .inspect_err(|e| tracing::error!("put failed: {e}"))
        .map_err(Error::from_string)?
        .execute()
        .await
        .inspect_err(|e| tracing::error!("execute failed: {e}"))
        .map_err(Error::from_string)
}
