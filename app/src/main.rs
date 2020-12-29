mod clipboard;
mod plugin;

use std::fs::read_dir;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use crate::clipboard::Clipboard;
use crate::plugin::PluginCollection;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
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
    let path = Path::new("./plugins")
        .canonicalize()
        .map_err(|e| Error::IO(e))?;

    for entry in read_dir(&path).map_err(|e| Error::IO(e))? {
        let entry = entry.map_err(|e| Error::IO(e))?;

        unsafe {
            let plugin = plugins
                .load(path.join(entry.path()))
                .map_err(|e| Error::Plugin(e))?;

            println!(
                "Plugin Loaded: {} ({})",
                plugin.name,
                entry.file_name().to_str().unwrap(),
            );
        }
    }

    let mut previous = String::new();
    let mut sequence = 0u32;

    loop {
        sleep(Duration::from_secs(1));

        let clipboard = Clipboard::open().map_err(Error::Clipboard)?;
        let seq = clipboard.get_sequence_number().map_err(Error::Clipboard)?;

        if seq == sequence {
            continue;
        }

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

        sequence = seq;
    }
}
