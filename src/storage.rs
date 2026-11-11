// Handles saving/loading/serializing into/from JSON

use crate::game::GameEntry;

use directories::ProjectDirs;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn get_app_data_dir() -> Option<PathBuf> {
    ProjectDirs::from("io.github", "chris_3345", "Cartulary")
        .map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
}

fn get_library_path() -> Option<PathBuf> {
    let data_dir = get_app_data_dir()?;
    let library_path = data_dir.join("library.json");

    Some(library_path)
}

/// Saves a slice of game entries atomically to DATA_DIR/library.json (writes to a tmp file first)
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
        serde_json::to_writer_pretty(&mut writer, entries)?;
        writer.flush()?;
        file.sync_all()?;
    }

    // Rename atomically so nothing gets corrupted
    fs::rename(tmp_path, path)?;

    Ok(())
}
