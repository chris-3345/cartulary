use chrono::NaiveDate;
use iced::widget::text_editor;
use uuid::Uuid;

use crate::{GameEntry, Platform, Rating, Status, Store};

pub struct AppState {
    pub games: Vec<GameEntry>,
    pub page: Page,
    pub credits: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenDetails(Uuid),
    OpenLibrary,
    OpenCreate,
    OpenCredits,
    EditEntry(Uuid, FieldEdit),
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
    Year(Option<u16>),
    DateStarted(Option<NaiveDate>),
    DateFinished(Option<NaiveDate>),
    Notes(String),
    Status(Status),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    Library,
    Create,
    Detail(Uuid),
    Credits,
}
