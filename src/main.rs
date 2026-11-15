// Stop console from showing up on Windows release builds
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod constants;
mod game;
mod icon;
mod message;
mod storage;
mod ui;

use iced::widget::{container, operation, row, text_editor};
use iced::{Element, Fill, Font, Size, Task, padding, window};

#[cfg(target_os = "linux")]
use crate::constants::APP_ID;

use crate::constants::{CREDITS, ICON_FONT, TITLE_FONT};
use crate::game::{GameEntry, Platform, Rating, Status, Store};
use crate::message::{AppState, EntryDraft, FieldEdit, Message, Page};
use crate::storage::{load_entries, save_entries};
use crate::ui::{page_content, sidebar};

// Helper functions ==========================

/// Formats a String like 20260408 into 2026-04-08
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

/// Applies a single field edit to a draft (used by both Create and Edit pages).
fn apply_edit(draft: &mut EntryDraft, field_edit: FieldEdit) -> Task<Message> {
    match field_edit {
        FieldEdit::Title(new_title) => {
            draft.title = new_title;
            Task::none()
        }
        FieldEdit::Year(new_year) => {
            // If it's not a number then ignore the message
            if new_year.chars().all(|c| c.is_ascii_digit()) && new_year.len() <= 4 {
                draft.year = new_year;
            }
            Task::none()
        }
        FieldEdit::Status(new_status) => {
            draft.status = new_status;
            Task::none()
        }
        FieldEdit::Rating(new_rating) => {
            draft.rating = new_rating;
            Task::none()
        }
        FieldEdit::Platform(new_platform) => {
            draft.platform = new_platform;
            Task::none()
        }
        FieldEdit::Store(new_store) => {
            draft.store = new_store;
            Task::none()
        }
        FieldEdit::DateStarted(new_date) => {
            draft.date_started = format_date_input(&new_date);
            operation::move_cursor_to_end("date_started")
        }
        FieldEdit::DateFinished(new_date) => {
            draft.date_finished = format_date_input(&new_date);
            operation::move_cursor_to_end("date_finished")
        }
        FieldEdit::Notes(action) => {
            draft.notes.perform(action);
            Task::none()
        }
    }
}

// Iced update & view ==========================

fn update(state: &mut AppState, message: Message) -> iced::Task<Message> {
    match message {
        // Page-change messages
        Message::OpenDetails(id) => {
            let Some(game) = state.games.iter().find(|g| g.id == id) else {
                return Task::none();
            };
            state.edit_draft = EntryDraft::from_entry(game);
            state.edit_error = None;
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
        // Edit/create/delete messages
        Message::EditDraft(edit) => apply_edit(&mut state.draft, edit),
        Message::EditDetail(edit) => apply_edit(&mut state.edit_draft, edit),
        Message::SaveEdit(id) => {
            // Build updated entry
            let mut updated = match state.edit_draft.to_entry() {
                Ok(entry) => entry,
                Err(msg) => {
                    state.edit_error = Some(msg);
                    return Task::none();
                }
            };
            updated.id = id; // keep the original UUID

            // Find game in the vec
            let Some(index) = state.games.iter().position(|g| g.id == id) else {
                return Task::none();
            };

            // Swap in the new one but keep the old one in case the saving fales
            let old = std::mem::replace(&mut state.games[index], updated);

            match save_entries(&state.games) {
                Ok(()) => state.edit_error = None,
                Err(e) => {
                    state.games[index] = old;
                    state.edit_error = Some(format!("Couldn't save: {e}"));
                }
            }

            Task::none()
        }
        Message::RevertEdit(id) => {
            if let Some(game) = state.games.iter().find(|g| g.id == id) {
                state.edit_draft = EntryDraft::from_entry(game);
            }
            state.edit_error = None;
            Task::none()
        }
        Message::CreateEntry => {
            match state.draft.to_entry() {
                Ok(entry) => {
                    state.games.push(entry);
                    match save_entries(&state.games) {
                        Ok(()) => {
                            state.draft = EntryDraft::default();
                            state.page = Page::Library;
                            state.create_error = None;
                        }
                        Err(e) => {
                            state.games.pop();
                            state.create_error = Some(format!("Couldn't save: {}", e));
                        }
                    }
                }
                Err(msg) => {
                    state.create_error = Some(format!("Couldn't save: {}", msg));
                }
            }

            Task::none()
        }
        Message::DeleteEntry(id) => {
            let Some(game) = state.games.iter().find(|g| g.id == id) else {
                return Task::none();
            };

            let dialog = rfd::AsyncMessageDialog::new()
                .set_level(rfd::MessageLevel::Warning)
                .set_title("Delete entry?")
                .set_description(format!("Delete \"{}\"? This can't be undone.", game.title))
                .set_buttons(rfd::MessageButtons::YesNo)
                .show();

            Task::perform(dialog, move |answer| {
                if answer == rfd::MessageDialogResult::Yes {
                    Message::ConfirmDelete(id)
                } else {
                    Message::Unused
                }
            })
        }
        Message::ConfirmDelete(id) => {
            // Get the index of the game
            let Some(index) = state.games.iter().position(|g| g.id == id) else {
                return Task::none(); // already gone somehow
            };

            // Keep it as a variable in case saving fails
            let entry = state.games.remove(index);

            match save_entries(&state.games) {
                Ok(()) => state.page = Page::Library,
                Err(e) => {
                    state.games.insert(index, entry);
                    return Task::perform(
                        rfd::AsyncMessageDialog::new()
                            .set_level(rfd::MessageLevel::Error)
                            .set_title("Couldn't delete")
                            .set_description(format!("Couldn't save: {e}"))
                            .show(),
                        |_| Message::Unused,
                    );
                }
            }

            Task::none()
        }

        // No-op-ish messages
        Message::CreditsAction(action) => {
            // Allow selecting scrolling and copying but block typing/deleting
            if !action.is_edit() {
                state.credits.perform(action);
            }
            Task::none()
        }
        // See comment for this in message.rs (intentionally no-op)
        Message::Unused => Task::none(),
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
        min_size: Some(Size::new(890.0, 520.0)),
        ..Default::default()
    }
}

fn new() -> AppState {
    let games = match load_entries() {
        Ok(games) => games,
        Err(e) => {
            rfd::MessageDialog::new()
                .set_level(rfd::MessageLevel::Error)
                .set_title("Cartulary")
                .set_description(format!("Couldn't load your library: {}", e))
                .show();
            std::process::exit(1);
        }
    };
    AppState {
        games,
        page: Page::Library,
        credits: text_editor::Content::with_text(CREDITS),
        draft: EntryDraft::default(),
        create_error: None,
        edit_draft: EntryDraft::default(),
        edit_error: None,
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
