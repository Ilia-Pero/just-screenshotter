//! Where the program keeps its files.

use std::path::PathBuf;

use screenshotter_phrase::{Language, Phrase};
use screenshotter_store::{Failure, Folder, Listened, Settings, SettingsFile};
use screenshotter_upload::ServerFile;
use tauri::{AppHandle, Manager};

use crate::hotkey;

/// The folder for pictures. Not every Linux names it; then it is the usual one.
fn pictures(app: &AppHandle) -> Result<PathBuf, Failure> {
    app.path()
        .picture_dir()
        .or_else(|_| Ok(app.path().home_dir()?.join("Pictures")))
}

/// Settings until a person changes them.
pub fn initial(app: &AppHandle) -> Result<Settings, Failure> {
    let program = std::env::current_exe()?;
    Ok(Settings {
        hotkey: hotkey::INITIAL.to_owned(),
        folder: pictures(app)?.join("Screenshots"),
        language: program.parent().map_or_default(Language::installed),
        ears: Listened::default(),
    })
}

/// The file with the settings.
pub fn settings(app: &AppHandle) -> Result<SettingsFile, Failure> {
    Ok(SettingsFile::new(
        app.path().app_config_dir()?.join("settings.json"),
        initial(app)?,
    ))
}

/// The folder with screenshots.
pub fn folder(app: &AppHandle) -> Result<Folder, Failure> {
    Ok(Folder::new(
        settings(app)?.read()?.folder,
        Phrase::new("file").to_string(),
    ))
}

/// The folder with films: the same one, but names of files begin with another word.
pub fn films(app: &AppHandle) -> Result<Folder, Failure> {
    Ok(Folder::new(
        settings(app)?.read()?.folder,
        Phrase::new("movie").to_string(),
    ))
}

/// The file with the address of the server for screenshots.
pub fn server(app: &AppHandle) -> Result<ServerFile, Failure> {
    Ok(ServerFile::new(
        app.path().app_config_dir()?.join("server.json"),
    ))
}
