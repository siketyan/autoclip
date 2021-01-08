use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) polling_interval: u64,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            polling_interval: 1000,
        }
    }

    pub(crate) fn load(path: &PathBuf) -> Result<Self> {
        let file = File::open(path).map_err(Error::Io)?;
        let reader = BufReader::new(file);

        serde_yaml::from_reader(reader).map_err(Error::Yaml)
    }
}
