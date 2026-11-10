// The builders for Cartulary's sidebar and other UI.

use crate::{AppState, GameEntry, ICON_FONT, Message, Page, TITLE_FONT};
use iced::widget::{
    button, column, container, mouse_area, row, scrollable, space, svg, text, text_editor,
    text_input,
};
use iced::{Center, Element, Fill, Font, mouse, padding};

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
        Page::Create => container(
            column![
                text("Create an entry:").font(TITLE_FONT).size(36),
                text_input("Title", &state.draft.title)
                    .on_input(|t| Message::EditDraft(FieldEdit::Title(t))),
            ]
            .max_width(300)
            .spacing(12),
        )
        .into(),
        Page::Detail(id) => text(format!("Detail page for ID {}", id)).into(),
        Page::Credits => text_editor(&state.credits)
            .on_action(Message::CreditsAction)
            .font(Font::MONOSPACE)
            .size(12)
            .height(Fill)
            .into(),
    }
}

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
        format!("★ {}/10", r.value())
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
