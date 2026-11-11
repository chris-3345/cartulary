// Holds structs and enums used by Cartulary
// for game entries

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Status {
    #[default]
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
impl Status {
    pub const ALL: [Status; 4] = [
        Status::Playing,
        Status::Dropped,
        Status::Backlog,
        Status::Finished,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Platform {
    #[default]
    Pc,
    Nintendo,
    Xbox,
    PlayStation,
    Ios,
    Android,
    Other,
}
impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_name = match self {
            Platform::Pc => "PC",
            Platform::Nintendo => "Nintendo",
            Platform::Xbox => "Xbox",
            Platform::PlayStation => "PlayStation",
            Platform::Ios => "iOS",
            Platform::Android => "Android",
            Platform::Other => "Other",
        };
        write!(f, "{pretty_name}")
    }
}
impl Platform {
    pub const ALL: [Platform; 7] = [
        Platform::Pc,
        Platform::Nintendo,
        Platform::Xbox,
        Platform::PlayStation,
        Platform::Ios,
        Platform::Android,
        Platform::Other,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Store {
    #[default]
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
    AppleAppStore,
    GooglePlayStore,
    Other,
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
            Store::AppleAppStore => "Apple App Store",
            Store::GooglePlayStore => "Google Play Store",
            Store::Other => "Other",
        };

        write!(f, "{pretty_name}")
    }
}
impl Store {
    pub const ALL: [Store; 15] = [
        Store::Steam,
        Store::Gog,
        Store::Epic,
        Store::Itch,
        Store::XboxApp,
        Store::PlayStationStore,
        Store::EShop,
        Store::BattleNet,
        Store::Ea,
        Store::Ubisoft,
        Store::Humble,
        Store::AppleAppStore,
        Store::GooglePlayStore,
        Store::Physical,
        Store::Other,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

    pub const ALL: [Rating; 10] = [
        Rating(1),
        Rating(2),
        Rating(3),
        Rating(4),
        Rating(5),
        Rating(6),
        Rating(7),
        Rating(8),
        Rating(9),
        Rating(10),
    ];
}

impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "★ {}/5", self.0 as f32 / 2.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
