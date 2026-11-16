use chrono::NaiveDate;
use iced::widget::text_editor;
use uuid::Uuid;

use crate::{GameEntry, Platform, Rating, Status, Store};

pub struct AppState {
    pub games: Vec<GameEntry>,
    pub page: Page,
    pub credits: text_editor::Content,
    pub draft: EntryDraft,
    pub create_error: Option<String>,
    pub edit_draft: EntryDraft,
    pub edit_error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenDetails(Uuid),
    OpenLibrary,
    OpenCreate,
    OpenCredits,
    EditDetail(FieldEdit),
    SaveEdit(Uuid),
    RevertEdit(Uuid),
    EditDraft(FieldEdit),
    CreditsAction(text_editor::Action),
    CreateEntry,
    DeleteEntry(Uuid),
    ConfirmDelete(Uuid),

    // INTENTIONALLY NO-OP
    // For things that need to look active but are
    // just status indicators
    Unused,
}

#[derive(Debug, Clone)]
pub enum FieldEdit {
    Title(String),
    Platform(Platform),
    Store(Store),
    Rating(Option<Rating>),
    Year(String),
    DateStarted(String),
    DateFinished(String),
    Notes(text_editor::Action),
    Status(Status),
}

#[derive(Debug, Clone, Default)]
pub struct EntryDraft {
    // Rating, Store, Platform, and Status are dropdowns.
    pub title: String,
    pub platform: Platform,
    pub store: Store,
    pub rating: Option<Rating>,
    // Typed fields are Strings
    // Otherwise they wouldn't convert well halfway through typing
    pub year: String,
    pub date_started: String,
    pub date_finished: String,
    pub notes: text_editor::Content,
    pub status: Status,
}

impl EntryDraft {
    pub fn to_entry(&self) -> Result<GameEntry, String> {
        let id = Uuid::now_v7();
        let title = self.title.trim().to_string();
        let status = self.status;
        let rating = self.rating;
        let platform = self.platform;
        let store = self.store;

        let year = if self.year.is_empty() {
            None
        } else {
            let parsed = self
                .year
                .parse::<u16>()
                .map_err(|_| "Year is somehow not a number".to_string())?;
            Some(parsed)
        };

        let date_started = if self.date_started.is_empty() {
            None
        } else {
            let parsed = NaiveDate::parse_from_str(&self.date_started, "%Y-%m-%d")
                .map_err(|_| "Date started is not a real date".to_string())?;
            Some(parsed)
        };

        let date_finished = if self.date_finished.is_empty() {
            None
        } else {
            let parsed = NaiveDate::parse_from_str(&self.date_finished, "%Y-%m-%d")
                .map_err(|_| "Date finished is not a real date".to_string())?;
            Some(parsed)
        };

        let notes = self.notes.text();

        Ok(GameEntry {
            id,
            title,
            platform,
            store,
            rating,
            year,
            date_started,
            date_finished,
            notes,
            status,
        })
    }

    pub fn from_entry(game: &GameEntry) -> EntryDraft {
        EntryDraft {
            title: game.title.clone(),
            platform: game.platform,
            store: game.store,
            status: game.status,
            rating: game.rating,
            year: game.year.map(|y| y.to_string()).unwrap_or_default(),
            date_started: game.date_started.map(|d| d.to_string()).unwrap_or_default(),
            date_finished: game
                .date_finished
                .map(|d| d.to_string())
                .unwrap_or_default(),
            notes: text_editor::Content::with_text(&game.notes),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    Library,
    Create,
    Detail(Uuid),
    Credits,
}
