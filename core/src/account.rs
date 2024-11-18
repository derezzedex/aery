#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiotId {
    pub name: Option<String>,    // 3~16 chars
    pub tagline: Option<String>, // 3~5 chars
}

impl RiotId {
    pub fn new(name: impl ToString, tagline: impl ToString) -> Self {
        Self {
            name: Some(name.to_string()),
            tagline: Some(tagline.to_string()),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Puuid(String);

impl AsRef<str> for Puuid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub puuid: Puuid,
    pub riot_id: RiotId,
}
