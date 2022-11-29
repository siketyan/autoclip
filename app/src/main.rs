mod clipboard;
mod config;
mod installer;
mod platform;
mod plugin;

use clap::{App, Arg, SubCommand};

use std::fs::{create_dir_all, read_dir};
use std::path::{Path, PathBuf};
use std::process::exit;
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
    Clipboard(#[from] clipboard::Error),

    #[error("config error: {0}")]
    Config(#[from] config::Error),

    #[error("installer error: {0}")]
    Installer(#[from] installer::Error),

    #[error("platform-specific error: {0}")]
    Platform(#[from] platform::Error),

    #[error("plugin error: {0}")]
    Plugin(#[from] plugin::Error),

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

enum RunMode<'a> {
    Single(&'a Path),
    All,
}

fn load_single_plugin<P>(
    plugins: &mut PluginCollection,
    plugins_path: &PathBuf,
    entry: P,
) -> Result<()>
where
    P: AsRef<Path>,
{
    unsafe {
        let plugin = plugins
            .load(plugins_path.join(entry.as_ref()))
            .map_err(Error::Plugin)?;

        println!(
            "Plugin Loaded: {} ({})",
            plugin.name,
            entry.as_ref().file_name().unwrap().to_str().unwrap(),
        );
    }

    Ok(())
}

fn run(config: &Config, plugins_path: &PathBuf, run_mode: RunMode) -> Result<()> {
    let mut plugins = PluginCollection::new();

    if let RunMode::Single(entry) = run_mode {
        load_single_plugin(&mut plugins, plugins_path, entry)?;
    } else {
        for entry in read_dir(plugins_path)? {
            let entry = entry?;

            load_single_plugin(&mut plugins, plugins_path, entry.path())?;
        }
    }

    let mut previous = String::new();

    loop {
        sleep(Duration::from_millis(config.polling_interval));

        let mut clipboard = Clipboard::open()?;
        let contents = clipboard.read_text()?;

        if contents == previous {
            continue;
        }

        previous = contents.clone();

        // this only supports macOS now, so defaults to empty vec
        let types = platform::get_clipboard_types().unwrap_or_default();
        if let Some(ty) = types.iter().find(|ty| config.ignored_types.contains(ty)) {
            println!("Ignoring type: {}", ty);

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

fn execute() -> Result<()> {
    let config_path = dirs::config_dir()
        .ok_or(Error::ConfigDirNotFound)?
        .join("autoclip")
        .join("config.yaml");

    let config = if config_path.exists() {
        println!("Using config at {}", config_path.to_str().unwrap());

        Config::load(&config_path)?
    } else {
        println!("Using the default config");

        Config::new()
    };

    let plugins_path = dirs::data_local_dir()
        .ok_or(DataLocalDirNotFound)?
        .join("autoclip")
        .join("plugins");

    if !plugins_path.exists() {
        create_dir_all(&plugins_path)?;
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
        .subcommand(
            SubCommand::with_name("single")
                .about("Run a single plugin.")
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
                .map_err(|e| e.into())
        }
        Some("single") => {
            let plugin_name = matches
                .subcommand_matches("single")
                .unwrap()
                .value_of("plugin_name")
                .unwrap()
                .as_ref();

            run(&config, &plugins_path, RunMode::Single(plugin_name))
        }
        _ => run(&config, &plugins_path, RunMode::All),
    }
}

fn main() {
    if let Err(error) = execute() {
        eprintln!("Error: {}", error);
        exit(1);
    }
}
