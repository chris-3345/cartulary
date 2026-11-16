use iced::{Font, font};

pub const TITLE_FONT: Font = Font {
    weight: font::Weight::Bold,
    ..Font::with_name("Google Sans")
};

pub const ICON_FONT: Font = Font {
    ..Font::with_name("lucide")
};

pub const CREDITS: &str = include_str!("../CREDITS.txt");

// Note: When we allow the user to change the theme, the wordmark will change depending on the theme.
// (and will also probably not be in constants for that reason.)
// Currently, LightColors is used for the sake of contrast with Nord's defaults.
pub const WORDMARK: &[u8] =
    include_bytes!("../media/Wordmarks/Cartulary-Wordmark-NoTM-LightColors.svg");

pub const ICON_SVG: &[u8] = include_bytes!("../media/Icons/Cartulary-logo.svg");

#[cfg(target_os = "linux")]
pub const APP_ID: &str = "io.github.chris_3345.Cartulary";
