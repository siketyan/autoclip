mod clipboard;
mod config;
mod installer;
mod plugin;

use clap::{App, Arg, SubCommand};

use std::fs::{create_dir_all, read_dir};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use crate::clipboard::Clipboard;
use crate::config::Config;
use crate::installer::Installer;
use crate::plugin::PluginCollection;
use crate::Error::DataLocalDirNotFound;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("config directory not found")]
    ConfigDirNotFound,

    #[error("local data directory not found")]
    DataLocalDirNotFound,

    #[error("clipboard error: {0}")]
    Clipboard(#[from] crate::clipboard::Error),

    #[error("config error: {0}")]
    Config(#[from] crate::config::Error),

    #[error("installer error: {0}")]
    Installer(#[from] crate::installer::Error),

    #[error("plugin error: {0}")]
    Plugin(#[from] crate::plugin::Error),

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

fn run(config: &Config, plugins_path: &PathBuf) -> Result<()> {
    let mut plugins = PluginCollection::new();

    for entry in read_dir(plugins_path).map_err(Error::IO)? {
        let entry = entry.map_err(Error::IO)?;

        unsafe {
            let plugin = plugins
                .load(plugins_path.join(entry.path()))
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
        sleep(Duration::from_millis(config.polling_interval));

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

fn main() -> Result<()> {
    let config_path = dirs::config_dir()
        .ok_or(Error::ConfigDirNotFound)?
        .join("autoclip")
        .join("config.yaml");

    let config = if config_path.exists() {
        println!("Using config at {}", config_path.to_str().unwrap());

        Config::load(&config_path).map_err(Error::Config)?
    } else {
        println!("Using the default config");

        Config::new()
    };

    let plugins_path = dirs::data_local_dir()
        .ok_or(DataLocalDirNotFound)?
        .join("autoclip")
        .join("plugins");

    if !plugins_path.exists() {
        create_dir_all(&plugins_path).map_err(Error::IO)?;
    }

    let matches = App::new("autoclip")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("install")
                .about("Installs a plugin.")
                .arg(Arg::with_name("plugin_name").required(true)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("install") => {
            let installer = Installer::new(None);
            let plugin_name = matches
                .subcommand_matches("install")
                .unwrap()
                .value_of("plugin_name")
                .unwrap();

            installer
                .install(plugin_name, &plugins_path)
                .map_err(Error::Installer)
        }
        _ => run(&config, &plugins_path),
    }
}
