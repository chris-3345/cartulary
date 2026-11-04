use iced::{Font, font};

pub const TITLE_FONT: Font = Font {
    weight: font::Weight::Bold,
    ..Font::with_name("Google Sans")
};

pub const ICON_FONT: Font = Font {
    ..Font::with_name("lucide")
};

pub const CREDITS: &str = include_str!("../CREDITS.txt");
