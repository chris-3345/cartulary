// Handles saving/loading/serializing into/from JSON

use crate::game::GameEntry;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufWriter, ErrorKind, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Save format version
const FORMAT_VERSION: u32 = 1;

/// What's written (borrows so don't need to copy)
#[derive(Serialize)]
struct LibraryFileOut<'a> {
    format_version: u32,
    games: &'a [GameEntry],
}

/// What's read (games become app state so we own)
#[derive(Deserialize)]
struct LibraryFileIn {
    format_version: u32,
    games: Vec<GameEntry>,
}

/// Settings for the app
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub secondary_folder: Option<PathBuf>,

    // Default is Nord
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            secondary_folder: None,
            theme: "Nord".to_string(),
        }
    }
}

/// Path getters
fn get_app_data_dir() -> Option<PathBuf> {
    ProjectDirs::from("io.github", "chris_3345", "Cartulary")
        .map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
}

fn get_library_path() -> Option<PathBuf> {
    let data_dir = get_app_data_dir()?;
    let library_path = data_dir.join("library.json");

    Some(library_path)
}

fn get_settings_path() -> Option<PathBuf> {
    Some(get_app_data_dir()?.join("settings.json"))
}

/// Saves the Settings struct to DATA_DIR/settings.json
pub fn save_settings(settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_settings_path().ok_or("couldn't find settings file")?;
    let tmp_path = path.with_extension("json.tmp");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write to tmp file
    {
        let file = File::create(&tmp_path)?;
        let mut writer = BufWriter::new(&file);
        let file_data = settings;
        serde_json::to_writer_pretty(&mut writer, &file_data)?;
        writer.flush()?;
        file.sync_all()?;
    }

    // Atomic rename
    fs::rename(tmp_path, path)?;

    Ok(())
}

/// Loads settings. Always returns usable settings (defaults if needed),
/// plus a warning message if something went wrong.
pub fn load_settings() -> (Settings, Option<String>) {
    let Some(path) = get_settings_path() else {
        return (
            Settings::default(),
            Some("Couldn't find app data dir".into()),
        );
    };

    let contents = match fs::read_to_string(&path) {
        Ok(s) => s,
        // First launch or settings just not created yet
        Err(e) if e.kind() == ErrorKind::NotFound => return (Settings::default(), None),
        // Couldn't read it but leave the file alone because it could be fine
        Err(e) => {
            return (
                Settings::default(),
                Some(format!(
                    "Couldn't read settings.json: {e}. Using defaults for now."
                )),
            );
        }
    };

    match serde_json::from_str::<Settings>(&contents) {
        Ok(settings) => (settings, None),
        Err(e) => {
            let broken = path.with_file_name("settings.broken.json");
            let warning = match fs::rename(&path, &broken) {
                Ok(()) => format!(
                    "settings.json couldn't be read ({e}), so your settings were reset. \
                     The old file was moved to {}.",
                    broken.display()
                ),
                Err(rename_err) => format!(
                    "settings.json couldn't be read ({e}) and couldn't be moved \
                     ({rename_err}). Using defaults for now."
                ),
            };
            (Settings::default(), Some(warning))
        }
    }
}

/// Saves a slice of game entries atomically to `DATA_DIR/library.json` (writes to a tmp file first)
pub fn save_entries(entries: &[GameEntry]) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_library_path().ok_or("couldn't find app data dir")?;
    let tmp_path = path.with_extension("json.tmp");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write to the tmp file
    {
        let file = File::create(&tmp_path)?;
        let mut writer = BufWriter::new(&file);
        let file_data = LibraryFileOut {
            format_version: FORMAT_VERSION,
            games: entries,
        };
        serde_json::to_writer_pretty(&mut writer, &file_data)?;
        writer.flush()?;
        file.sync_all()?;
    }

    // Rename atomically so nothing gets corrupted
    fs::rename(tmp_path, path)?;

    Ok(())
}

/// Loads game entries
pub fn load_entries() -> Result<Vec<GameEntry>, Box<dyn std::error::Error>> {
    let path = get_library_path().ok_or("couldn't find app data dir")?;

    let contents = match fs::read_to_string(&path) {
        // Read OK
        Ok(s) => s,

        // Doesn't exist; return an empty library
        // Library will be created on first save
        Err(e) if e.kind() == ErrorKind::NotFound => {
            return Ok(Vec::new());
        }

        // Couldn't read
        Err(e) => return Err(e.into()),
    };

    // Empty/whitespace only
    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    match serde_json::from_str::<LibraryFileIn>(&contents) {
        Ok(file) if file.format_version > FORMAT_VERSION => Err(format!(
            "library.json was saved by a newer version of Cartulary (format {}, this version understands up to {}). Update Cartulary to open it.",
            file.format_version, FORMAT_VERSION
        )
        .into()),

        Ok(file) => Ok(file.games),

        // Not valid JSON at all
        Err(e) if e.is_syntax() || e.is_eof() => {
            let name = rename_bad_lib(&path)?;
            Err(format!(
                "library.json is malformed: {}. Renamed to {}. Reload Cartulary to start a new library, or fix the old one and rename it back to library.json.",
                e,
                name.display()
            )
            .into())
        }

        // Not valid structure
        Err(e) => {
            let name = rename_bad_lib(&path)?;
            Err(format!(
                "library.json doesn't match the expected format: {}. Renamed to {}. Reload Cartulary to start a new library, or fix the old one and rename it back to library.json.",
                e,
                name.display()
            )
            .into())
        }
    }
}

/// Moves a bad library.json aside so it isn't lost, returns the new filename
fn rename_bad_lib(path: &Path) -> std::io::Result<PathBuf> {
    let name = format!("library_BAD_{}.json", Uuid::now_v7());
    fs::rename(path, path.with_file_name(&name))?;
    Ok(path.with_file_name(&name))
}
