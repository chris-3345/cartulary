// The builders for Cartulary's sidebar and other UI.

use crate::game::{Platform, Rating, Status, Store};
use crate::{AppState, GameEntry, ICON_FONT, Message, Page, TITLE_FONT};
use iced::widget::{
    button, column, container, mouse_area, pick_list, row, scrollable, space, svg, text,
    text_editor, text_input,
};
use iced::{Center, Element, Fill, Font, mouse, padding};

use crate::constants::{WORDMARK_FOR_DARK, WORDMARK_FOR_LIGHT};
use crate::message::FieldEdit;

// ======== SIDEBAR ========
/// Builds the sidebar
pub fn sidebar(state: &AppState) -> Element<'_, Message> {
    let wordmark = if state.theme.extended_palette().is_dark {
        WORDMARK_FOR_DARK
    } else {
        WORDMARK_FOR_LIGHT
    };
    let page = state.page;

    let detail_button: Element<'_, Message> = if matches!(page, Page::Detail(_)) {
        nav_button('\u{e4f0}', "Details", true, Message::Unused)
    } else {
        space().into()
    };

    container(
        column![
            container(svg(svg::Handle::from_memory(wordmark)).width(175))
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
            nav_button(
                '\u{e154}',
                "Settings",
                page == Page::Settings,
                Message::OpenSettings,
            ),
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
fn nav_button(icon: char, label: &str, active: bool, msg: Message) -> Element<'_, Message> {
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
        Page::Create => create_page(state),
        Page::Detail(id) => edit_page(state, id),
        Page::Credits => text_editor(&state.credits)
            .on_action(Message::CreditsAction)
            .font(Font::MONOSPACE)
            .size(12)
            .height(Fill)
            .into(),
        Page::Settings => settings_page(state),
    }
}

// ======== CREATE PAGE ========
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
                    rating_picker(state.draft.rating, |r| Message::EditDraft(
                        FieldEdit::Rating(r)
                    )),
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
                create_button(state),
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
        state
            .create_error
            .as_ref()
            .map(|err| text(err).style(text::danger)),
    ]
    .spacing(4)
    .into()
}

// ======== SHARED HELPER FUNCTIONS ========
/// Rating `pick_list` with a ✕ button to clear it back to "no rating".
fn rating_picker<'a>(
    current: Option<Rating>,
    on_change: impl Fn(Option<Rating>) -> Message + 'a,
) -> Element<'a, Message> {
    let clear_msg = on_change(None);

    let clear = button(text("✕")).style(button::secondary);
    let clear = if current.is_some() {
        clear.on_press(clear_msg)
    } else {
        clear
    };

    row![
        pick_list(&Rating::ALL[..], current, move |r| on_change(Some(r)))
            .width(Fill)
            .placeholder("Rating"),
        clear,
    ]
    .spacing(4)
    .width(Fill)
    .into()
}

// ======== EDIT PAGE ========
/// Helper function for the edit page's buttons (save/revert/delete + save error)
fn edit_buttons(state: &AppState, id: uuid::Uuid) -> Element<'_, Message> {
    let save = button(text("Save"));
    let save = if state.edit_draft.title.trim().is_empty() {
        save
    } else {
        save.on_press(Message::SaveEdit(id))
    };

    column![
        row![
            save,
            button(text("Revert"))
                .style(button::secondary)
                .on_press(Message::RevertEdit(id)),
            space::horizontal(),
            button(text("Delete"))
                .style(button::danger)
                .on_press(Message::DeleteEntry(id)),
        ]
        .spacing(8),
        state
            .edit_error
            .as_ref()
            .map(|err| text(err).style(text::danger)),
    ]
    .spacing(4)
    .into()
}

/// Edit page builder
fn edit_page(state: &AppState, id: uuid::Uuid) -> Element<'_, Message> {
    let Some(game) = state.games.iter().find(|g| g.id == id) else {
        return text("Game not found").into();
    };
    let draft = &state.edit_draft;

    scrollable(
        container(
            column![
                // Heading uses the saved title so it doesn't change while typing
                text(&game.title).font(TITLE_FONT).size(44),
                row![
                    text_input("Title (required)", &draft.title)
                        .on_input(|t| Message::EditDetail(FieldEdit::Title(t))),
                    text_input("Year (e.g. '2020')", &draft.year)
                        .on_input(|t| Message::EditDetail(FieldEdit::Year(t))),
                ]
                .spacing(6),
                row![
                    text_input("Date started (YYYY-MM-DD)", &draft.date_started)
                        .id("date_started")
                        .on_input(|t| Message::EditDetail(FieldEdit::DateStarted(t))),
                    text_input("Date finished (YYYY-MM-DD)", &draft.date_finished)
                        .id("date_finished")
                        .on_input(|t| Message::EditDetail(FieldEdit::DateFinished(t))),
                ]
                .spacing(6),
                row![
                    pick_list(&Status::ALL[..], Some(draft.status), |s| {
                        Message::EditDetail(FieldEdit::Status(s))
                    })
                    .width(Fill),
                    rating_picker(state.draft.rating, |r| Message::EditDetail(
                        FieldEdit::Rating(r)
                    )),
                    pick_list(&Platform::ALL[..], Some(draft.platform), |p| {
                        Message::EditDetail(FieldEdit::Platform(p))
                    })
                    .width(Fill),
                    pick_list(&Store::ALL[..], Some(draft.store), |s| {
                        Message::EditDetail(FieldEdit::Store(s))
                    })
                    .width(Fill),
                ]
                .spacing(6),
                text_editor(&draft.notes)
                    .min_height(150)
                    .max_height(300)
                    .on_action(|t| Message::EditDetail(FieldEdit::Notes(t))),
                edit_buttons(state, id),
            ]
            .max_width(600)
            .spacing(12),
        )
        .center_x(Fill),
    )
    .into()
}

// ======== GAME CARD ========
/// Builds a game card UI element from a `GameEntry` struct
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

// ======== SETTINGS PAGE ========

// Third party licenses menu will eventually be moved into this settings page
fn settings_page(state: &AppState) -> Element<'_, Message> {
    let folder_text = match &state.settings.secondary_folder {
        Some(path) => path.display().to_string(),
        None => "Not set".to_string(),
    };

    let settings = column![
        text("Settings").font(TITLE_FONT).size(36),
        setting_row(
            "Theme",
            "Changes the colors of the whole app.",
            pick_list(iced::Theme::ALL, Some(&state.theme), Message::ThemeChanged).width(200),
        ),
        setting_row(
            "Secondary library folder",
            folder_text,
            button(
                row![text('\u{E0D7}').font(ICON_FONT), text("Browse…")]
                    .spacing(6)
                    .align_y(Center),
            )
            .style(button::secondary)
            .padding([4, 10])
            .on_press(Message::PickSecondaryFolder),
        ),
    ]
    .max_width(600)
    .spacing(24);

    let licenses = button(
        row![
            text("\u{e4a3}").font(ICON_FONT),
            text("Third Party Licenses").size(12),
        ]
        .spacing(8)
        .align_y(Center),
    )
    .padding([4, 8])
    .style(button::secondary)
    .on_press(Message::OpenCredits);

    column![
        scrollable(container(settings).center_x(Fill)).height(Fill),
        container(licenses)
            .center_x(Fill)
            .padding(padding::bottom(10)),
    ]
    .height(Fill)
    .into()
}

/// A label + small description on the left, a control on the right.
fn setting_row<'a>(
    label: &'a str,
    description: impl Into<String>,
    control: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    row![
        column![
            text(label),
            text(description.into()).size(13).style(text::secondary),
        ]
        .spacing(2)
        .width(Fill),
        control.into(),
    ]
    .spacing(12)
    .align_y(Center)
    .into()
}
