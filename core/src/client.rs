use std::sync::Arc;

#[derive(Clone)]
pub struct Client(Arc<riven::RiotApi>);

impl Client {
    pub fn new(key: String) -> Self {
        Client(Arc::new(riven::RiotApi::new(key)))
    }
}

impl AsRef<riven::RiotApi> for Client {
    fn as_ref(&self) -> &riven::RiotApi {
        self.0.as_ref()
    }
}
