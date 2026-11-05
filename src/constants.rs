use iced::widget::svg;
use iced::{Font, font};

pub const TITLE_FONT: Font = Font {
    weight: font::Weight::Bold,
    ..Font::with_name("Google Sans")
};

pub const ICON_FONT: Font = Font {
    ..Font::with_name("lucide")
};

pub const CREDITS: &str = include_str!("../CREDITS.txt");

pub const WORDMARK: &[u8] = include_bytes!("../media/Playshelf-Wordmark.svg");

pub const ICON_SVG: &[u8] = include_bytes!("../media/Playshelf-logo-dark-bg.svg");

pub const APP_ID: &str = "io.github.chris_3345.Playshelf";
