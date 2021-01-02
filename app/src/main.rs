mod clipboard;
mod plugin;

use std::fs::{create_dir_all, read_dir};
use std::thread::sleep;
use std::time::Duration;

use crate::clipboard::Clipboard;
use crate::plugin::PluginCollection;
use crate::Error::DataLocalDirNotFound;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("local data directory not found")]
    DataLocalDirNotFound,

    #[error("clipboard error")]
    Clipboard(crate::clipboard::Error),

    #[error("plugin error")]
    Plugin(crate::plugin::Error),

    #[error("I/O error")]
    IO(std::io::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut plugins = PluginCollection::new();
    let path = dirs::data_local_dir()
        .ok_or(DataLocalDirNotFound)?
        .join("autoclip")
        .join("plugins");

    for entry in read_dir(&path).map_err(Error::IO)? {
        let entry = entry.map_err(Error::IO)?;

        unsafe {
            let plugin = plugins
                .load(path.join(entry.path()))
                .map_err(Error::Plugin)?;

            println!(
                "Plugin Loaded: {} ({})",
                plugin.name,
                entry.file_name().to_str().unwrap(),
            );
        }
    }

    let mut previous = String::new();

    loop {
        sleep(Duration::from_secs(1));

        let mut clipboard = Clipboard::open().map_err(Error::Clipboard)?;
        let contents = clipboard.read_text().map_err(Error::Clipboard)?;

        if contents == previous {
            continue;
        }

        if let Some(output) = plugins.dispatch_on_clip(&contents) {
            clipboard
                .write_text(&output.as_str())
                .map_err(Error::Clipboard)?;

            println!("Wrote: {} -> {}", contents, output);
            previous = output;
        }
    }
}
