mod constants;
mod game;
mod message;
mod ui;

use iced::widget::{container, row, text_editor};
use iced::{Element, Fill, Font, padding};
use uuid::Uuid;

use crate::constants::{CREDITS, ICON_FONT, TITLE_FONT};
use crate::game::{GameEntry, Platform, Rating, Status, Store};
use crate::message::{AppState, Message, Page};
use crate::ui::{page_content, sidebar};

// Iced update & view ==========================

fn update(state: &mut AppState, message: Message) {
    match message {
        // Page-change messages
        Message::OpenDetails(id) => {
            state.page = Page::Detail(id);
        }
        Message::OpenLibrary => {
            state.page = Page::Library;
        }
        Message::OpenCreate => {
            state.page = Page::Create;
        }
        Message::OpenCredits => {
            state.page = Page::Credits;
        }
        Message::CreditsAction(action) => {
            // Allow selecting scrolling and copying but block typing/deleting
            if !action.is_edit() {
                state.credits.perform(action);
            }
        }
        // See above comment (in the Message enum)
        Message::Unused => (),

        // Non-page messages
        Message::EditEntry(id, field_to_edit) => {
            todo!()
        }
    }
}

fn view(state: &AppState) -> Element<'_, Message> {
    // game_card(&state.games[0])

    let sidebar = sidebar(state);
    let page_content = container(page_content(state))
        .padding(padding::left(20).top(10).right(20))
        .width(Fill)
        .height(Fill);

    row![sidebar, page_content,].into()
}

// Init ==========================

fn theme(_state: &AppState) -> iced::Theme {
    iced::Theme::TokyoNight
}

fn new() -> AppState {
    AppState {
        games: vec![
            GameEntry {
                id: Uuid::now_v7(),
                title: "Hades".to_string(),
                platform: Platform::Pc,
                store: Store::Steam,
                status: Status::Finished,
                rating: Rating::new(9),
                year: Some(2020),
                date_started: None,
                date_finished: None,
                notes: String::new(),
            },
            GameEntry {
                id: Uuid::now_v7(),
                title: "Portal 2".to_string(),
                platform: Platform::Pc,
                store: Store::Steam,
                status: Status::Finished,
                rating: Rating::new(10),
                year: Some(2011),
                date_started: None,
                date_finished: None,
                notes: String::new(),
            },
        ],
        page: Page::Library,
        credits: text_editor::Content::with_text(CREDITS),
    }
}

fn main() -> iced::Result {
    iced::application(new, update, view)
        .title("Playshelf")
        .font(include_bytes!("../fonts/google-sans/GoogleSans-Regular.ttf").as_slice())
        .font(include_bytes!("../fonts/google-sans/GoogleSans-Bold.ttf").as_slice())
        .font(include_bytes!("../fonts/lucide-font-1.66.0/font/lucide.ttf").as_slice())
        .default_font(Font::with_name("Google Sans"))
        .theme(theme)
        .run()
}
