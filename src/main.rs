use chrono::NaiveDate;
use iced::widget::{
    button, column, container, mouse_area, row, scrollable, space, text, text_editor,
};
use iced::{Center, Element, Fill, Font, font, mouse, padding};
use uuid::Uuid;

mod structs;

use crate::structs::{GameEntry, Platform, Rating, Status, Store};

const TITLE_FONT: Font = Font {
    weight: font::Weight::Bold,
    ..Font::with_name("Google Sans")
};

const ICON_FONT: Font = Font {
    ..Font::with_name("lucide")
};

const CREDITS: &str = include_str!("../CREDITS.txt");

struct AppState {
    games: Vec<GameEntry>,
    page: Page,
    credits: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
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
enum FieldEdit {
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
enum Page {
    Library,
    Create,
    Detail(Uuid),
    Credits,
}

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

fn theme(_state: &AppState) -> iced::Theme {
    iced::Theme::TokyoNight
}

fn new() -> AppState {
    AppState {
        games: vec![GameEntry {
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
        }],
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

// CUSTOM UI BUILDER THINGS ==========================

/// Builds the sidebar
fn sidebar(state: &AppState) -> Element<'_, Message> {
    let detail_button: Element<'_, Message> = if let Page::Detail(id) = state.page {
        button(row![text("\u{e4f0}").font(ICON_FONT), " Details"])
            .width(Fill)
            .on_press(Message::Unused)
            .into()
    } else {
        space().into()
    };

    container(
        column![
            button(row![text("\u{e0e9}").font(ICON_FONT), " Library"])
                .width(Fill)
                .on_press(Message::OpenLibrary),
            button(row![text("\u{e173}").font(ICON_FONT), " New Entry"])
                .width(Fill)
                .on_press(Message::OpenCreate),
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

/// Builds the page content
fn page_content(state: &AppState) -> Element<'_, Message> {
    // Get a the first game entry
    let game = &state.games[0];
    let id = game.id;

    match state.page {
        Page::Library => {
            // Eventually build game cards here. For now, placeholder:
            button("Click for edit game test")
                .on_press(Message::OpenDetails(id))
                .into()
        }
        Page::Create => text("Create page").into(),
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
