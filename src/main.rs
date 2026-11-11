// Stop console from showing up on Windows release builds
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod constants;
mod game;
mod icon;
mod message;
mod storage;
mod ui;

use iced::widget::{container, operation, row, text_editor};
use iced::{Element, Fill, Font, Task, padding, window};
use uuid::Uuid;

use crate::constants::{APP_ID, CREDITS, ICON_FONT, TITLE_FONT};
use crate::game::{GameEntry, Platform, Rating, Status, Store};
use crate::message::{AppState, EntryDraft, FieldEdit, Message, Page};
use crate::ui::{page_content, sidebar};

// Helper functions ==========================
fn format_date_input(raw: &str) -> String {
    let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).take(8).collect();

    let mut formatted = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i == 4 || i == 6 {
            formatted.push('-');
        }
        formatted.push(c);
    }

    formatted
}
// Iced update & view ==========================

fn update(state: &mut AppState, message: Message) -> iced::Task<Message> {
    match message {
        // Page-change messages
        Message::OpenDetails(id) => {
            state.page = Page::Detail(id);
            Task::none()
        }
        Message::OpenLibrary => {
            state.page = Page::Library;
            Task::none()
        }
        Message::OpenCreate => {
            state.page = Page::Create;
            Task::none()
        }
        Message::OpenCredits => {
            state.page = Page::Credits;
            Task::none()
        }
        Message::EditDraft(field_edit) => match field_edit {
            FieldEdit::Title(new_title) => {
                state.draft.title = new_title;
                Task::none()
            }
            FieldEdit::Year(new_year) => {
                // If it's not a number then ignore the message
                if new_year.chars().all(|c| c.is_ascii_digit()) && new_year.len() <= 4 {
                    state.draft.year = new_year;
                }
                Task::none()
            }
            FieldEdit::Status(new_status) => {
                state.draft.status = new_status;
                Task::none()
            }
            FieldEdit::Rating(new_rating) => {
                state.draft.rating = new_rating;
                Task::none()
            }
            FieldEdit::Platform(new_platform) => {
                state.draft.platform = new_platform;
                Task::none()
            }
            FieldEdit::Store(new_store) => {
                state.draft.store = new_store;
                Task::none()
            }
            FieldEdit::DateStarted(new_date) => {
                state.draft.date_started = format_date_input(&new_date);
                operation::move_cursor_to_end("date_started")
            }
            FieldEdit::DateFinished(new_date) => {
                state.draft.date_finished = format_date_input(&new_date);
                operation::move_cursor_to_end("date_finished")
            }
            _ => {
                todo!()
            }
        },
        Message::CreditsAction(action) => {
            // Allow selecting scrolling and copying but block typing/deleting
            if !action.is_edit() {
                state.credits.perform(action);
            }
            Task::none()
        }
        // See comment for this in message.rs (intentionally no-op)
        Message::Unused => Task::none(),

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
