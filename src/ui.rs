// The builders for Cartulary's sidebar and other UI.

use crate::game::{Platform, Rating, Status, Store};
use crate::{AppState, GameEntry, ICON_FONT, Message, Page, TITLE_FONT};
use iced::widget::{
    button, column, container, mouse_area, pick_list, row, scrollable, space, svg, text,
    text_editor, text_input,
};
use iced::{Center, Element, Fill, Font, mouse, padding};
use uuid::Uuid;

use crate::constants::WORDMARK;
use crate::message::FieldEdit;

// ======== SIDEBAR ========
/// Builds the sidebar
pub fn sidebar(state: &AppState) -> Element<'_, Message> {
    let page = state.page;

    let detail_button: Element<'_, Message> = if matches!(page, Page::Detail(_)) {
        nav_button('\u{e4f0}', "Details", true, Message::Unused)
    } else {
        space().into()
    };

    container(
        column![
            container(svg(svg::Handle::from_memory(WORDMARK)).width(175))
                .center_x(Fill)
                .padding(padding::bottom(10).top(5)),
            nav_button(
                '\u{e0e9}',
                "Library",
                page == Page::Library,
                Message::OpenLibrary
            ),
            nav_button(
                '\u{e173}',
                "New Entry",
                page == Page::Create,
                Message::OpenCreate
            ),
            detail_button,
            space::vertical(),
            button(
                row![
                    text("\u{e4a3}").font(ICON_FONT),
                    text("   Third Party Licenses").size(12)
                ]
                .align_y(Center),
            )
            .width(Fill)
            .padding([4, 8])
            .style(button::secondary)
            .on_press(Message::OpenCredits),
        ]
        .spacing(15)
        .height(Fill),
    )
    .height(Fill)
    .padding(padding::right(15).left(10).bottom(10).top(10))
    .width(200)
    .style(|theme: &iced::Theme| {
        let bg = theme.extended_palette().background.base.color;
        container::Style {
            background: Some(
                iced::Color {
                    r: bg.r * 0.8,
                    g: bg.g * 0.8,
                    b: bg.b * 0.8,
                    ..bg
                }
                .into(),
            ),
            ..Default::default()
        }
    })
    .into()
}

/// Sidebar nav button; uses the primary style when `active`, otherwise a muted one.
fn nav_button<'a>(icon: char, label: &'a str, active: bool, msg: Message) -> Element<'a, Message> {
    button(row![text(icon).font(ICON_FONT), text(label)].spacing(8))
        .width(Fill)
        .style(if active {
            button::primary
        } else {
            button::secondary
        })
        .on_press(msg)
        .into()
}

// ======== MAIN PAGE CONTENT ========
/// Builds the page content
pub fn page_content(state: &AppState) -> Element<'_, Message> {
    match state.page {
        Page::Library => {
            let cards = state.games.iter().map(game_card);

            scrollable(
                row(cards).spacing(12).wrap(), // Make the cards a grid instead of a long row
            )
            .into()
        }
        Page::Create => create_page(&state),
        Page::Detail(id) => detail_page(state, id),
        Page::Credits => text_editor(&state.credits)
            .on_action(Message::CreditsAction)
            .font(Font::MONOSPACE)
            .size(12)
            .height(Fill)
            .into(),
    }
}

/// Details page builder
fn detail_page(state: &AppState, id: uuid::Uuid) -> Element<'_, Message> {
    let Some(game) = state.games.iter().find(|g| g.id == id) else {
        return text("Game not found.").into();
    };

    let or_dash = |v: Option<String>| v.unwrap_or_else(|| "—".to_string());

    let title = if let Some(y) = game.year {
        format!("{} ({})", game.title, y)
    } else {
        game.title.clone()
    };

    let notes = if game.notes.trim().is_empty() {
        "No notes"
    } else {
        &game.notes
    };

    let info = container(
        column![
            field_row("Status", game.status.to_string()),
            field_row("Rating", or_dash(game.rating.map(|r| r.to_string()))),
            field_row("Platform", game.platform.to_string()),
            field_row("Store", game.store.to_string()),
            field_row("Started", or_dash(game.date_started.map(|d| d.to_string()))),
            field_row(
                "Finished",
                or_dash(game.date_finished.map(|d| d.to_string()))
            ),
        ]
        .spacing(6),
    )
    .padding(12)
    .width(Fill)
    .style(container::rounded_box);

    scrollable(
        container(
            column![
                text(title).font(TITLE_FONT).size(44),
                info,
                text("Notes").style(text::secondary),
                container(text(notes))
                    .padding(12)
                    .width(Fill)
                    .style(container::rounded_box),
                button("Delete entry")
                    .style(button::danger)
                    .on_press(Message::DeleteEntry(id)),
            ]
            .max_width(600)
            .spacing(12),
        )
        .center_x(Fill),
    )
    .into()
}

/// One "Label    value" line on the Details page.
fn field_row<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    row![text(label).style(text::secondary).width(120), text(value),].into()
}

/// The Create page
fn create_page(state: &AppState) -> Element<'_, Message> {
    scrollable(
        container(
            column![
                text("Create an entry:").font(TITLE_FONT).size(36),
                row![
                    text_input("Title (required)", &state.draft.title)
                        .on_input(|t| Message::EditDraft(FieldEdit::Title(t))),
                    text_input("Year (e.g. '2020')", &state.draft.year)
                        .on_input(|t| Message::EditDraft(FieldEdit::Year(t))),
                ]
                .spacing(6),
                row![
                    text_input("Date started (YYYY-MM-DD)", &state.draft.date_started)
                        .id("date_started")
                        .on_input(|t| Message::EditDraft(FieldEdit::DateStarted(t))),
                    text_input("Date finished (YYYY-MM-DD)", &state.draft.date_finished)
                        .id("date_finished")
                        .on_input(|t| Message::EditDraft(FieldEdit::DateFinished(t))),
                ]
                .spacing(6),
                row![
                    pick_list(&Status::ALL[..], Some(state.draft.status), |s| {
                        Message::EditDraft(FieldEdit::Status(s))
                    })
                    .width(Fill),
                    pick_list(&Rating::ALL[..], state.draft.rating, |r| {
                        Message::EditDraft(FieldEdit::Rating(Some(r)))
                    })
                    .width(Fill)
                    .placeholder("Rating"),
                    pick_list(&Platform::ALL[..], Some(state.draft.platform), |p| {
                        Message::EditDraft(FieldEdit::Platform(p))
                    })
                    .width(Fill),
                    pick_list(&Store::ALL[..], Some(state.draft.store), |s| {
                        Message::EditDraft(FieldEdit::Store(s))
                    })
                    .width(Fill),
                ]
                .spacing(6),
                text_editor(&state.draft.notes)
                    .min_height(150)
                    .max_height(300)
                    .on_action(|t| Message::EditDraft(FieldEdit::Notes(t))),
                create_button(&state),
            ]
            .max_width(600)
            .spacing(12),
        )
        .center_x(Fill),
    )
    .into()
}

/// Handles the create button and its errors
fn create_button(state: &AppState) -> Element<'_, Message> {
    let create_button = button(text("Create"));

    column![
        if state.draft.title.trim().is_empty() {
            create_button
        } else {
            create_button.on_press(Message::CreateEntry)
        },
        if let Some(err) = &state.create_error {
            Some(text(err).style(text::danger))
        } else {
            None
        },
    ]
    .spacing(4)
    .into()
}

/// Edit page
fn edit_page(state: &AppState) -> Element<'_, Message> {}

/// Builds a game card UI element from a GameEntry struct
fn game_card(game: &GameEntry) -> Element<'_, Message> {
    // Prepare the text first, then build the UI.
    let title = if let Some(y) = game.year {
        format!("{} ({})", game.title, y)
    } else {
        game.title.clone()
    };

    // Empty string when unrated: on a card, missing info just isn't shown.
    let rating = if let Some(r) = game.rating {
        format!("{r}")
    } else {
        String::new()
    };

    let card = container(
        column![
            text(title).size(20).font(TITLE_FONT),
            // Muted secondary text for the meta line.
            text(format!("{} · {}", game.platform, game.store))
                .size(14)
                .style(text::secondary),
            // Bottom line: status on the left, rating pushed to the right.
            // space::horizontal() is an invisible spacer that fills all leftover width.
            row![
                text(game.status.to_string()),
                space::horizontal(),
                text(rating),
            ],
        ]
        .spacing(6),
    )
    .padding(12)
    .width(220)
    .style(container::rounded_box);

    // Wrap the whole card so clicking anywhere on it opens the detail page.
    mouse_area(card)
        .on_press(Message::OpenDetails(game.id))
        .interaction(mouse::Interaction::Pointer)
        .into()
}
