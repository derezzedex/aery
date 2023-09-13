pub struct Client(pub riven::RiotApi);

impl Client {
    pub fn new(key: String) -> Self {
        Client(riven::RiotApi::new(key))
    }
}
