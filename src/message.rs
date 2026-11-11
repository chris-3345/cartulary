use iced::widget::text_editor;
use uuid::Uuid;

use crate::{GameEntry, Platform, Rating, Status, Store};

pub struct AppState {
    pub games: Vec<GameEntry>,
    pub page: Page,
    pub credits: text_editor::Content,
    pub draft: EntryDraft,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenDetails(Uuid),
    OpenLibrary,
    OpenCreate,
    OpenCredits,
    EditEntry(Uuid, FieldEdit),
    EditDraft(FieldEdit),
    CreditsAction(text_editor::Action),

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    Library,
    Create,
    Detail(Uuid),
    Credits,
}
