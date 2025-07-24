use riven::models::account_v1;
use std::fmt;

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
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

impl fmt::Display for RiotId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name.as_deref().unwrap_or("missing");
        let tagline = self.tagline.as_deref().unwrap_or("404");
        write!(f, "{name}#{tagline}")
    }
}

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct Puuid(String);

impl AsRef<str> for Puuid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct Account {
    pub puuid: Puuid,
    pub riot_id: RiotId,
}

impl Account {
    #[cfg(feature = "dummy")]
    pub fn dummy(riot_id: RiotId) -> Self {
        let name = riot_id.name.as_deref().unwrap_or("foo");
        let tagline = riot_id.tagline.as_deref().unwrap_or("bar");

        Self {
            puuid: Puuid(format!("{name}-{tagline}")),
            riot_id,
        }
    }
}

impl From<account_v1::Account> for Account {
    fn from(account: account_v1::Account) -> Self {
        Self {
            puuid: Puuid(account.puuid),
            riot_id: RiotId {
                name: account.game_name,
                tagline: account.tag_line,
            },
        }
    }
}
