use iced::{Font, font};

pub const TITLE_FONT: Font = Font {
    weight: font::Weight::Bold,
    ..Font::with_name("Google Sans")
};

pub const ICON_FONT: Font = Font {
    ..Font::with_name("lucide")
};

pub const CREDITS: &str = include_str!("../CREDITS.txt");

pub const WORDMARK: &[u8] =
    include_bytes!("../media/Wordmarks/Cartulary-Wordmark-NoTM-DarkColors.svg");

pub const ICON_SVG: &[u8] = include_bytes!("../media/Icons/Cartulary-logo.svg");

pub const APP_ID: &str = "io.github.chris_3345.Cartulary";
