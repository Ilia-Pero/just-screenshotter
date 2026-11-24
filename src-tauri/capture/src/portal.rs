//! The screen of Linux.

use std::fs;

use ashpd::desktop::screenshot::Screenshot;
use ashpd::zbus::Connection;
use url::Url;

use crate::{Failure, Patch, Picture, Screen};

/// The screen of Linux under Wayland. Nothing but the portal
/// `org.freedesktop.portal.Screenshot` can show it; GNOME asks for permission the first time.
///
/// @todo #1 Nobody has run this on Linux yet. Check it on Ubuntu 26.04 with one and two
///  monitors and with a fractional scale.
pub struct Portal;

impl Portal {
    /// The picture of the whole desktop. The portal leaves it in a file nobody needs afterwards.
    fn picture() -> Result<Vec<u8>, Failure> {
        let shot = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                // A connection of its own for every picture: a shared one would outlive
                // the thread it was made in.
                Screenshot::request()
                    .connection(Some(Connection::session().await?))
                    .interactive(false)
                    .modal(false)
                    .send()
                    .await?
                    .response()
            })?;
        let file = Url::parse(shot.uri().as_str())?
            .to_file_path()
            .map_err(|()| format!("The portal gave no file: {}", shot.uri().as_str()))?;
        let png = fs::read(&file)?;
        fs::remove_file(&file)?;
        Ok(png)
    }
}

impl Screen for Portal {
    fn pngs(&self, patches: &[Patch]) -> Result<Vec<Vec<u8>>, Failure> {
        let png = Self::picture()?;
        if patches.len() < 2 {
            return Ok(vec![png]);
        }
        let desktop = patches
            .iter()
            .copied()
            .reduce(|whole, patch| whole.joined(&patch))
            .ok_or("There are no monitors")?;
        let whole = Picture::new(png).pixels()?;
        patches
            .iter()
            .map(|patch| {
                whole
                    .cut(&patch.on(&desktop, whole.width, whole.height))?
                    .png()
            })
            .collect()
    }
}
