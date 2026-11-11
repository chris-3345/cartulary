// Stop console from showing up on Windows release builds
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod constants;
mod game;
mod icon;
mod message;
mod storage;
mod ui;

use iced::widget::{container, row, text_editor};
use iced::{Element, Fill, Font, padding, window};
use uuid::Uuid;

use crate::constants::{APP_ID, CREDITS, ICON_FONT, TITLE_FONT};
use crate::game::{GameEntry, Platform, Rating, Status, Store};
use crate::message::{AppState, EntryDraft, FieldEdit, Message, Page};
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
        Message::EditDraft(field_edit) => match field_edit {
            FieldEdit::Title(new_title) => {
                state.draft.title = new_title;
            }
            FieldEdit::Year(new_year) => {
                // If it's not a number then ignore the message
                if new_year.chars().all(|c| c.is_ascii_digit()) && new_year.len() <= 4 {
                    state.draft.year = new_year;
                }
            }
            FieldEdit::Status(new_status) => {
                state.draft.status = new_status;
            }
            FieldEdit::Rating(new_rating) => {
                state.draft.rating = new_rating;
            }
            _ => {
                todo!("other fields in EditDraft")
            }
        },
        Message::CreditsAction(action) => {
            // Allow selecting scrolling and copying but block typing/deleting
            if !action.is_edit() {
                state.credits.perform(action);
            }
        }
        // See comment for this in message.rs (intentionally no-op)
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
    iced::Theme::CatppuccinMocha
}

fn window_settings() -> window::Settings {
    window::Settings {
        icon: icon::load_icon(),
        #[cfg(target_os = "linux")]
        platform_specific: window::settings::PlatformSpecific {
            application_id: APP_ID.to_string(),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn new() -> AppState {
    AppState {
        games: Vec::new(),
        page: Page::Library,
        credits: text_editor::Content::with_text(CREDITS),
        draft: EntryDraft::default(),
    }
}

fn main() -> iced::Result {
    iced::application(new, update, view)
        .title("Cartulary")
        .font(include_bytes!("../fonts/google-sans/GoogleSans-Regular.ttf").as_slice())
        .font(include_bytes!("../fonts/google-sans/GoogleSans-Bold.ttf").as_slice())
        .font(include_bytes!("../fonts/lucide-font-1.66.0/font/lucide.ttf").as_slice())
        .default_font(Font::with_name("Google Sans"))
        .theme(theme)
        .window(window_settings())
        .run()
}
