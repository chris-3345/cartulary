// Holds most structs and pub enums except for AppState and Messages (and anything else relating specifically to the app's UI).

use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Playing,
    Finished,
    Dropped,
    Backlog, // Have but not playing
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Playing => "Playing",
            Self::Backlog => "Backlog",
            Self::Dropped => "Dropped",
            Self::Finished => "Finished",
        };
        write!(f, "{label}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Platform {
    Pc,
    Nintendo,
    Xbox,
    PlayStation,
    Other(String),
}
impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_name = match self {
            Platform::Pc => "PC",
            Platform::Nintendo => "Nintendo",
            Platform::Xbox => "Xbox",
            Platform::PlayStation => "PlayStation",
            Self::Other(name) => name.as_str(),
        };
        write!(f, "{pretty_name}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Store {
    Steam,
    Gog,
    Epic,
    Itch,
    XboxApp,
    PlayStationStore,
    EShop,
    BattleNet,
    Ea,
    Ubisoft,
    Humble,
    Physical,
    Other(String),
}
impl std::fmt::Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_name = match self {
            Store::Steam => "Steam",
            Store::Gog => "GOG",
            Store::Epic => "Epic Games",
            Store::Itch => "Itch.io",
            Store::XboxApp => "Xbox App",
            Store::PlayStationStore => "PlayStation Store",
            Store::EShop => "eShop",
            Store::BattleNet => "Battle.net",
            Store::Ea => "EA",
            Store::Ubisoft => "Ubisoft",
            Store::Humble => "Humble",
            Store::Physical => "Physical",
            Self::Other(name) => name.as_str(),
        };

        write!(f, "{pretty_name}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rating(u8);

impl Rating {
    pub fn new(value: u8) -> Option<Rating> {
        if 0 < value && value <= 10 {
            Some(Rating(value))
        } else {
            None
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct GameEntry {
    pub id: Uuid,
    pub title: String,
    pub platform: Platform,
    pub store: Store,
    pub rating: Option<Rating>,
    pub year: Option<u16>,
    pub date_started: Option<NaiveDate>,
    pub date_finished: Option<NaiveDate>,
    pub notes: String,
    pub status: Status,
}
